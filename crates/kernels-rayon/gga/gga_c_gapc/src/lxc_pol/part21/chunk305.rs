//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 305/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk305(t11: f64, t19: f64, t1154: f64, t1161: f64, t357: f64, t21: f64, t410: f64, t1165: f64, t1167: f64, t1169: f64, t363: f64, t347: f64) -> (f64, f64, f64, f64) {
    let t1195 = 1.0_f64/f64::sqrt(t11);
    let t1196 = t1195 * t19;
    let t1197 = t1196 * t1154;
    let t1199 = t357 * t1161;
    let t1201 = t21 * t410;
    let t1203 = -0.42198333333333333333e0_f64 * t1165 + 0.84396666666666666666e0_f64 * t1167 + 0.39862222222222222223e0_f64 * t1169 + 0.68258333333333333333e-1_f64 * t1197 + 0.13651666666666666667e0_f64 * t1199 + 0.13692777777777777778e0_f64 * t1201;
    let t1204 = t1203 * t363;
    let t1206 = 1.0_f64 * t347 * t1204;
    (t1197, t1199, t1201, t1206)
}
