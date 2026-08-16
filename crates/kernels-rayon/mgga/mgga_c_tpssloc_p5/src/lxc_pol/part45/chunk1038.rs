//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1038/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1038(t114552: f64, t2039: f64, t1873: f64, t84097: f64, t115241: f64, t115783: f64, t115785: f64, t115788: f64, t115790: f64, t115792: f64, t22461: f64, t2363: f64, t23917: f64, t31532: f64, t6517: f64, t671: f64, t7056: f64, t8446: f64, t90041: f64, t90044: f64) -> f64 {
    let t115796 = 2.0_f64 * t114552 * t2039;
    let t115802 = 2.0_f64 * t84097 * t1873;
    let t115809 = 4.0_f64 * t115241 * t671 + 4.0_f64 * t2039 * t90041 + 2.0_f64 * t2039 * t90044 + 4.0_f64 * t22461 * t7056 + 2.0_f64 * t2363 * t31532 + 2.0_f64 * t23917 * t6517 + t115783 + t115785 + t115788 + t115790 + t115792 + t115796 + t115802 + t8446;
    t115809
}
