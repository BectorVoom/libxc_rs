//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 310/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk310(t1227: f64, t372: f64, t1165: f64, t1167: f64, t1169: f64, t1197: f64, t1199: f64, t1201: f64, t374: f64, t381: f64, t373: f64, t1225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1229 = 0.11696446794910408142e1_f64 * t372 * t1227;
    let t1236 = -0.57538888888888888889e0_f64 * t1165 + 0.11507777777777777778e1_f64 * t1167 + 0.40256666666666666667e0_f64 * t1169 + 0.366775e-1_f64 * t1197 + 0.73355e-1_f64 * t1199 + 0.137975e0_f64 * t1201;
    let t1238 = t374 * t1236 * t381;
    let t1240 = 0.58482233974552040708e0_f64 * t372 * t1238;
    let t1241 = t373 * t373;
    let t1242 = 1.0_f64 / t1241;
    let t1243 = t1242 * t1225;
    (t1229, t1236, t1238, t1240, t1242, t1243)
}
