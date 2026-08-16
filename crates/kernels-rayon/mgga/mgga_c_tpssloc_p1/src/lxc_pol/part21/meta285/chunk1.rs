//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1579/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1579(t10352: f64, t973: f64, t1036: f64, t3078: f64, t1032: f64, t3082: f64, t2393: f64, t374: f64, t376: f64, t370: f64, t3158: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10353 = t973 * t10352;
    let t10370 = t3078 * t1036;
    let t10372 = t1032 * t3082;
    let t10375 = t374 * t2393 * t376;
    let t10377 = t370 * t10375 / 10368.0_f64;
    let t10381 = t964 * t3158;
    (t10353, t10370, t10372, t10375, t10377, t10381)
}
