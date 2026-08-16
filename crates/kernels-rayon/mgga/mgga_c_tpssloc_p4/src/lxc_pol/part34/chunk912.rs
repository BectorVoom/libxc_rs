//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 912/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk912(t10214: f64, t21468: f64, t20234: f64, t2980: f64, t977: f64, t21126: f64, t4518: f64, t13909: f64, t17784: f64, t17809: f64, t21430: f64, t21433: f64, t21447: f64, t21453: f64, t21459: f64, t21463: f64, t2986: f64, t973: f64) -> f64 {
    let t21469 = t10214 * t21468;
    let t21472 = t2980 * t20234;
    let t21473 = t977 * t21472;
    let t21476 = t4518 * t21126;
    let t21479 = 0.16666666666666666666e-2_f64 * t2986 * t21430 - 0.83333333333333333331e-3_f64 * t2986 * t21433 - 0.83333333333333333332e-3_f64 * t973 * t21447 - 0.55555555555555555554e-3_f64 * t17809 - 0.24999999999999999999e-2_f64 * t973 * t21453 - 0.83333333333333333332e-3_f64 * t973 * t21459 + 0.27777777777777777777e-3_f64 * t973 * t21463 + 0.37037037037037037036e-3_f64 * t17784 + 0.55555555555555555554e-3_f64 * t13909 + 0.86419753086419753084e-3_f64 * t973 * t21469 + 0.16666666666666666666e-2_f64 * t973 * t21473 - 0.16666666666666666666e-2_f64 * t2986 * t21476;
    t21479
}
