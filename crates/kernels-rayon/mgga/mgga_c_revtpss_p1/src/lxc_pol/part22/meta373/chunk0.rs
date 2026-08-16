//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1919/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1919(t247: f64, t3368: f64, t3634: f64, t1261: f64, t3636: f64, t3647: f64, t3367: f64, t414: f64) -> (f64, f64, f64, f64) {
    let t13089 = t247 * t3634 * t3368;
    let t13090 = t1261 * t13089;
    let t13092 = t3647 * t3636;
    let t13099 = 1.0_f64 / t414 / t3367;
    (t13089, t13090, t13092, t13099)
}
