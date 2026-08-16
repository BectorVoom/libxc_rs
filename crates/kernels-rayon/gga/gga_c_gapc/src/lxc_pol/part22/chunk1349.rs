//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1349/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1349(t12055: f64, t4908: f64, t11043: f64, t3449: f64, t10544: f64, t8601: f64, t31754: f64, t3268: f64, t2468: f64, t3828: f64, t2470: f64, t10086: f64, t3565: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36314 = 4.0_f64 * t4908 * t12055;
    let t36316 = 2.0_f64 * t11043 * t3449;
    let t36318 = 2.0_f64 * t8601 * t10544;
    let t36320 = 4.0_f64 * t31754 * t3268;
    let t36321 = t3828 * t2468;
    let t36323 = 2.0_f64 * t36321 * t2470;
    let t36324 = t3565 * t10086;
    (t36314, t36316, t36318, t36320, t36323, t36324)
}
