//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 811/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk811(t3005: f64, t6406: f64, t971: f64, t3013: f64, t3020: f64, t4612: f64, t4706: f64, t6328: f64, t6332: f64, t6336: f64, t6341: f64, t6343: f64, t6375: f64, t6377: f64, t6381: f64, t6384: f64, t6387: f64) -> (f64, f64) {
    let t6408 = t3005 * t6406 * t971;
    let t6423 = -0.1294625e1_f64 * t6341 + 0.258925e1_f64 * t6343 + t3013 + 0.20128333333333333334e0_f64 * t4612 - 0.20128333333333333333e0_f64 * t6328 + 0.60385e0_f64 * t6332 - 0.301925e0_f64 * t6336 + 0.82524375e-1_f64 * t6375 + 0.16504875e0_f64 * t6377 + t3020 + 0.11038e0_f64 * t4706 - 0.27595e-1_f64 * t6381 + 0.16557e0_f64 * t6384 - 0.82785e-1_f64 * t6387;
    (t6408, t6423)
}
