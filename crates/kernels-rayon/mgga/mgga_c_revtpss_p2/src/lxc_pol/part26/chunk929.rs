//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 929/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk929(t126: f64, t373: f64, t828: f64, t3119: f64, t3115: f64, t1086: f64, t3057: f64, t3090: f64, t1043: f64, t3059: f64, t1045: f64, t3117: f64) -> (f64, f64, f64, f64) {
    let t11921 = t126 * t373;
    let t11922 = t828 * t11921;
    let t11923 = t11922 * t3119;
    let t11924 = t3115 * t11923;
    let t11926 = t3057 * t1086;
    let t11927 = t11926 * t3090;
    let t11928 = t3059 * t1043;
    let t11929 = t11928 * t1045;
    let t11930 = t3117 * t11929;
    (t11924, t11927, t11928, t11930)
}
