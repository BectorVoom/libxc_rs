//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1208/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1208(t188: f64, t46965: f64, t3377: f64, t11977: f64, t524: f64, t13778: f64, t2487: f64, t6985: f64, t1445: f64, t46920: f64, t597: f64, t42395: f64, t42398: f64, t42401: f64, t42405: f64, t42407: f64, t42413: f64, t42416: f64, t42421: f64) -> f64 {
    let t48187 = t188 * t46965;
    let t48188 = t48187 * t3377;
    let t48190 = t524 * t11977;
    let t48191 = t48190 * t3377;
    let t48194 = t2487 * t6985 * t13778;
    let t48198 = 0.11502877786176224903e2_f64 * t597 * t1445 * t46920;
    let t48200 = -t42395 - t42398 - 0.10725146985555128001e1_f64 * t48188 - 0.10725146985555128001e1_f64 * t48191 - 0.25561950635947166451e0_f64 * t48194 + t48198 - t42401 - t42405 + t42407 - t42413 + 0.42603251059911944084e-1_f64 * t42416 - t42421;
    t48200
}
