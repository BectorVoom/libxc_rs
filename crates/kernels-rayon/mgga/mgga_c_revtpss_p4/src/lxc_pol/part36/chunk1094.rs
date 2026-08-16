//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1094/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1094(t1225: f64, t22671: f64, t1012: f64, t13006: f64, t22688: f64, t13027: f64, t13020: f64, t1774: f64, t6628: f64, t3604: f64, t3720: f64, t3611: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24816 = t1225 * t22671;
    let t24817 = t1012 * t24816;
    let t24820 = t13006 * t22688;
    let t24821 = t1012 * t24820;
    let t24826 = t13027 * t22688;
    let t24827 = t1012 * t24826;
    let t24830 = t13020 * t22688;
    let t24831 = t1012 * t24830;
    let t24834 = t1774 * t6628;
    let t24835 = t24834 * t3604;
    let t24836 = t3720 * t24835;
    let t24839 = t24834 * t3611;
    (t24817, t24821, t24827, t24831, t24834, t24836, t24839)
}
