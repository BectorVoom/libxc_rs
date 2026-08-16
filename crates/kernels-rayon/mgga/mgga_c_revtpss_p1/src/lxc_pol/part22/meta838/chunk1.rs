//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2967/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2967(t3938: f64, t48919: f64, t9816: f64, t9818: f64, t13847: f64, t13848: f64, t4057: f64, t13962: f64, t9962: f64, t13845: f64, t5675: f64, t9840: f64) -> (f64, f64, f64, f64, f64) {
    let t48922 = t9816 * t9818 * t48919 * t3938;
    let t48929 = t9816 * t13847 * t13848 * t4057;
    let t48937 = t9962 * t13962;
    let t48941 = t13845 * t13847 * t48919 * t5675;
    let t48945 = t13845 * t13847 * t13848 * t9840;
    (t48922, t48929, t48937, t48941, t48945)
}
