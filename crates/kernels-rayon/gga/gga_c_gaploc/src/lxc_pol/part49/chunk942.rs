//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 942/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk942(t42199: f64, t12986: f64, t2464: f64, t2487: f64, t35204: f64, t9346: f64, t204: f64, t41965: f64, t587: f64, t10156: f64, t3377: f64, t524: f64) -> (f64, f64, f64, f64, f64) {
    let t42200 = 0.11502877786176224903e1_f64 * t42199;
    let t42202 = t2487 * t2464 * t12986;
    let t42203 = 0.63904876589867916128e-1_f64 * t42202;
    let t42205 = 0.21450293971110256001e2_f64 * t35204 * t9346;
    let t42208 = 0.92023022289409799224e1_f64 * t587 * t204 * t41965;
    let t42210 = t524 * t10156 * t3377;
    (t42200, t42203, t42205, t42208, t42210)
}
