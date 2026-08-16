//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1037/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1037(t13148: f64, t17708: f64, t1209: f64, t489: f64, t3623: f64, t370: f64, t3566: f64, t13142: f64, t13127: f64, t1778: f64, t3682: f64, t372: f64, t5268: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17709 = t13148 * t17708;
    let t17727 = t1209 * t489;
    let t17728 = t3623 * t370;
    let t17729 = t17727 * t17728;
    let t17735 = t3566 * t489;
    let t17736 = t17735 * t17728;
    let t17747 = t13142 * t17708;
    let t17753 = t13127 * t17708;
    let t17792 = t1778 * t3682;
    let t17799 = t372 * t5268;
    (t17709, t17729, t17736, t17747, t17753, t17792, t17799)
}
