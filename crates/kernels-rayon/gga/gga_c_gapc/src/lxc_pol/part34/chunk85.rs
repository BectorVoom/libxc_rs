//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 85/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk85(t11: f64, t14: f64, t17: f64, t25: f64, t231: f64, t33: f64, t56: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t233 = 1.0_f64 + 0.5137e-1_f64 * t11;
    let t238 = 0.705945e1_f64 * t14 + 0.1549425e1_f64 * t11 + 0.420775e0_f64 * t17 + 0.1562925e0_f64 * t25;
    let t241 = 1.0_f64 + 0.32164683177870697974e2_f64 / t238;
    let t242 = f64::ln(t241);
    let t247 = t231 * (-0.3109e-1_f64 * t233 * t242 + t33 - 0.19751789702565206229e-1_f64 * t56);
    let t249 = 0.19751789702565206229e-1_f64 * t231 * t56;
    let t252 = 0.149676e1_f64 + 0.89527e-3_f64 * t14 + 0.11799625e-1_f64 * t11;
    let t255 = 1.0_f64 + t14 * t252 / 2.0_f64;
    let t256 = t255 * t255;
    let t257 = 1.0_f64 / t256;
    (t233, t238, t241, t242, t247, t249, t252, t255, t256, t257)
}
