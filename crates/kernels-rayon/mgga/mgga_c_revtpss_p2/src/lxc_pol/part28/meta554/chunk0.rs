//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2005/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2005(t25305: f64, t92868: f64, t1032: f64, t2760: f64, t867: f64, t7063: f64, t7060: f64, t136: f64, t2457: f64, t7082: f64, t25299: f64, t212: f64, t25286: f64, t689: f64, t780: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92875 = 0.91399340044406952588e-2_f64 * t25305 * t92868;
    let t92888 = t2760 * t1032;
    let t92889 = t92888 * t867;
    let t92890 = t7063 * t92889;
    let t92891 = t92890 * t7060;
    let t92894 = t7082 * t136 * t2457;
    let t92895 = t25299 * t92894;
    let t92901 = t689 * t212 * t25286 * t780;
    (t92875, t92888, t92889, t92891, t92894, t92895, t92901)
}
