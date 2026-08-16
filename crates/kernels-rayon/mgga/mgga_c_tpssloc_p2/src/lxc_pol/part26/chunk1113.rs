//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1113/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1113(t252: f64, t776: f64, t829: f64, t6646: f64, t22986: f64, t6624: f64, t814: f64, t2627: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22987 = t252 * t776;
    let t22988 = t22987 * t829;
    let t22989 = t6646 * t22988;
    let t22990 = t22986 * t22989;
    let t22992 = t814 * t6624;
    let t22993 = t22992 * t829;
    let t22996 = t6604 * t2627;
    (t22987, t22988, t22989, t22990, t22992, t22993, t22996)
}
