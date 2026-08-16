//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1061/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1061(t11981: f64, t11984: f64, t11988: f64, t11992: f64, t11995: f64, t11998: f64, t10099: f64, t3568: f64, t1096: f64, t3622: f64, t2469: f64, t3832: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12269 = 0.35848176214430067276e-9_f64 * t11981;
    let t12270 = 0.33147827249531850013e-7_f64 * t11984;
    let t12271 = 0.34752370105806885418e-3_f64 * t11988;
    let t12272 = 0.4637672555408563478e-4_f64 * t11992;
    let t12273 = 0.4637672555408563478e-4_f64 * t11995;
    let t12274 = 0.38647271295071362317e-6_f64 * t11998;
    let t12281 = 2.0_f64 * t10099 * t3568;
    let t12285 = t3622 * t1096;
    let t12287 = 2.0_f64 * t2469 * t12285;
    let t12288 = t3832 * t972;
    (t12269, t12270, t12271, t12272, t12273, t12274, t12281, t12285, t12287, t12288)
}
