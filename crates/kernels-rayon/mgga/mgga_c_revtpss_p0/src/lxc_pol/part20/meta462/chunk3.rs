//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1759/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1759(t4059: f64, t9909: f64, t9812: f64, t9962: f64, t13845: f64, t46751: f64, t9818: f64, t9835: f64, t13847: f64, t9819: f64, t9840: f64, t9958: f64) -> (f64, f64, f64, f64, f64) {
    let t47229 = t9909 * t4059;
    let t47231 = t9962 * t9812;
    let t47235 = t13845 * t9818 * t46751 * t9835;
    let t47239 = t13845 * t13847 * t9819 * t9840;
    let t47245 = t9962 * t9958;
    (t47229, t47231, t47235, t47239, t47245)
}
