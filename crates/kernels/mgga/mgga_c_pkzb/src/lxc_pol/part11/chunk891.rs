//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 891/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk891<F: Float>(t10437: F, t4810: F, t2499: F, t3333: F, t10422: F, t27: F, t10415: F, t10418: F, t10423: F, t10428: F, t23: F, t28: F, t3324: F, t3330: F, t3334: F, t7: F, t980: F, t984: F) -> (F, F, F, F, F) {
    let t10438 = t4810 * t10437;
    let t10441 = t2499 * t3333;
    let t10444 = -t10422;
    let t10445 = t27 * t10444;
    let t10448 = -10.0 / 27.0 * t7 * t10415 + 10.0 / 3.0 * t7 * t10418 + 5.0 / 3.0 * t7 * t10423 - 1232.0 / 27.0 * t10428 * t28 + 440.0 / 9.0 * t3324 * t984 - 80.0 / 9.0 * t980 * t3330 - 40.0 / 3.0 * t980 * t3334 - 10.0 / 27.0 * t23 * t10438 + 10.0 / 3.0 * t23 * t10441 + 5.0 / 3.0 * t23 * t10445;
    (t10438, t10441, t10444, t10445, t10448)
}
