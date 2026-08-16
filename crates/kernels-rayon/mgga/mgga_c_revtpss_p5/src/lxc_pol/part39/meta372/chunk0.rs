//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1310/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1310(t3153: f64, t4866: f64, t4894: f64, t3117: f64, t3133: f64, t3154: f64, t4893: f64, t13396: f64, t4801: f64, t1042: f64, t11922: f64, t4911: f64) -> (f64, f64, f64, f64, f64) {
    let t15780 = t4866 * t3153;
    let t15781 = t15780 * t4894;
    let t15782 = t3117 * t15781;
    let t15785 = t3154 * t3133;
    let t15786 = t4893 * t15785;
    let t15787 = t3117 * t15786;
    let t15790 = t4801 * t13396;
    let t15791 = t1042 * t15790;
    let t15794 = t11922 * t4911;
    (t15780, t15782, t15787, t15791, t15794)
}
