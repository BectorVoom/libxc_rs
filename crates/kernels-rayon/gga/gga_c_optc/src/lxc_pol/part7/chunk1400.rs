//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1400/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1400(t26261: f64, t26264: f64, t26252: f64, t26258: f64, t26326: f64, t26328: f64, t26330: f64, t26332: f64, t26351: f64, t26354: f64, t26358: f64, t26314: f64, t26339: f64, t26343: f64, t26363: f64, t26365: f64, t26367: f64, t26369: f64, t26372: f64, t26376: f64, t26379: f64, t26382: f64, t26385: f64) -> (f64, f64) {
    let t27866 = 0.20106419753086419753e2_f64 * t26261;
    let t27867 = 0.20068888888888888889e-1_f64 * t26264;
    let t27875 = 0.28723456790123456789e1_f64 * t26252 + 0.2585111111111111111e2_f64 * t26258 + t27866 + t27867 - 0.51702222222222222221e1_f64 * t26326 - 0.34468148148148148146e1_f64 * t26328 - 0.6568e-2_f64 * t26351 + 0.6568e-2_f64 * t26354 + 0.10340444444444444444e2_f64 * t26330 + 0.8042567901234567901e1_f64 * t26332 + 0.14595555555555555556e-1_f64 * t26358;
    let t27888 = -0.57446913580246913579e1_f64 * t26339 - 0.19388333333333333333e1_f64 * t26343 - 0.821e-2_f64 * t26363 - 0.27366666666666666666e-2_f64 * t26365 + 0.3284e-2_f64 * t26367 + 0.14595555555555555556e-2_f64 * t26369 + 0.25851111111111111111e1_f64 * t26314 + 0.1642e-1_f64 * t26372 - 0.12771111111111111111e-2_f64 * t26376 - 0.12315e-2_f64 * t26379 - 0.3284e-2_f64 * t26382 + 0.9852e-2_f64 * t26385;
    (t27875, t27888)
}
