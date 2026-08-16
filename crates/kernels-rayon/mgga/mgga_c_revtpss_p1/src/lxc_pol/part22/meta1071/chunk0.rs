//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3836/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3836(t13845: f64, t13847: f64, t5675: f64, t73731: f64, t3938: f64, t9816: f64, t9818: f64, t13848: f64, t5659: f64, t22159: f64, t48836: f64, t22120: f64, t9962: f64) -> (f64, f64, f64, f64, f64) {
    let t73734 = t13845 * t13847 * t73731 * t5675;
    let t73738 = t9816 * t9818 * t73731 * t3938;
    let t73742 = t9816 * t13847 * t13848 * t5659;
    let t73744 = t48836 * t22159;
    let t73750 = t9962 * t22120;
    (t73734, t73738, t73742, t73744, t73750)
}
