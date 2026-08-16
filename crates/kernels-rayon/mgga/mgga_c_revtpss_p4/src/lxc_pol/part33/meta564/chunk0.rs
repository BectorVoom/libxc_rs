//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1963/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1963(t30735: f64, t7637: f64, t2142: f64, t6573: f64, t1769: f64, t8190: f64, t1774: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30736 = t7637 * t30735;
    let t30739 = t2142 * t6573;
    let t30740 = t7637 * t30739;
    let t30743 = t8190 * t1769;
    let t30744 = t7637 * t30743;
    let t30747 = t8190 * t1774;
    (t30736, t30739, t30740, t30743, t30744, t30747)
}
