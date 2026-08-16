//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2107/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2107(t13821: f64, t27940: f64, t13928: f64, t26028: f64, t241: f64, t820: f64, t94491: f64, t13807: f64, t13817: f64, t13991: f64, t13793: f64, t13786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98110 = t27940 * t13821;
    let t98112 = t26028 * t13928;
    let t98115 = t820 * t94491 * t241;
    let t98116 = t98115 * t13807;
    let t98118 = t27940 * t13817;
    let t98120 = t27940 * t13991;
    let t98122 = t27940 * t13793;
    let t98124 = t26028 * t13786;
    (t98110, t98112, t98116, t98118, t98120, t98122, t98124)
}
