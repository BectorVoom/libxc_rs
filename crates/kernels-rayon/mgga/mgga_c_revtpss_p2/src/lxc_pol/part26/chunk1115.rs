//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1115/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1115(t2257: f64, t775: f64, t2394: f64, t605: f64, t11054: f64, t30: f64, t10489: f64, t10627: f64, t198: f64, t268: f64, t41040: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92799 = t2257 * t775;
    let t92806 = t605 * t2394;
    let t92810 = t30 * t11054;
    let t92814 = t30 * t10489;
    let t92822 = t198 * t10627;
    let t92840 = t268 * t41040 * t837;
    (t92799, t92806, t92810, t92814, t92822, t92840)
}
