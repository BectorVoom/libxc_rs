//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1443/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1443(t10845: f64, t18531: f64, t18622: f64, t6016: f64, t853: f64, t18432: f64, t40336: f64, t5977: f64, t18441: f64, t9775: f64, t10716: f64, t18402: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t61572 = t10845 * t18531;
    let t61576 = t10845 * t18622;
    let t61579 = t853 * t6016;
    let t61623 = t40336 * t18432;
    let t61625 = t853 * t5977;
    let t61645 = t9775 * t18441;
    let t61675 = t10716 * t18402;
    (t61572, t61576, t61579, t61623, t61625, t61645, t61675)
}
