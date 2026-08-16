//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2447/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2447<F: Float>(t1015: F, t10472: F, t42559: F, t10870: F, t3048: F, t204: F, t376: F, t1020: F, t1023: F, t248: F, t10510: F, t3109: F) -> (F, F, F, F, F) {
    let t43211 = t10472 * t1015 * t42559;
    let t43214 = t3048 * t10870;
    let t43216 = t204 * t376;
    let t43219 = t1020 * t248 * t43216 * t1023;
    let t43221 = t3109 * t10510;
    (t43211, t43214, t43216, t43219, t43221)
}
