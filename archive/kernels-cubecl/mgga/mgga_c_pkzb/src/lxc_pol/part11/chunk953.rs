//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 953/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk953<F: Float>(t1429: F, t4803: F, t15: F, t20: F, t399: F, t3329: F, t983: F, t4810: F, t2499: F, t3333: F, t27: F, t10415: F, t10418: F, t23: F, t28: F, t3324: F, t3330: F, t3334: F, t7: F, t980: F, t984: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10421 = -t1429 - t4803;
    let t10422 = F::cast_from(3.0_f64) * t10421;
    let t10423 = t15 * t10422;
    let t10427 = F::cast_from(1.0_f64) / t20 / t399;
    let t10428 = sigma2 * t10427;
    let t10437 = t3329 * t983;
    let t10438 = t4810 * t10437;
    let t10441 = t2499 * t3333;
    let t10444 = -t10422;
    let t10445 = t27 * t10444;
    let t10448 = -F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t7 * t10415 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t7 * t10418 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7 * t10423 - F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t10428 * t28 + F::cast_from(440.0_f64) / F::cast_from(9.0_f64) * t3324 * t984 - F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t980 * t3330 - F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t980 * t3334 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t23 * t10438 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t23 * t10441 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t23 * t10445;
    (t10421, t10422, t10423, t10428, t10437, t10438, t10441, t10444, t10445, t10448)
}
