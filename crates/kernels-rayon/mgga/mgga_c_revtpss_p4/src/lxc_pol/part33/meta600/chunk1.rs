//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2024/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2024(t2453: f64, t25949: f64, t25946: f64, t25939: f64, t40270: f64, t10073: f64, t25920: f64, t25938: f64, t25898: f64, t94889: f64, t10115: f64, t2024: f64) -> (f64, f64, f64, f64, f64) {
    let t94913 = t2453 * t25949;
    let t94914 = t94913 * t25946;
    let t94917 = 0.96373646535613327356e-3_f64 * t40270 * t25939;
    let t94919 = t10073 * t25920 * t25938;
    let t94921 = t94889 * t25898;
    let t94931 = 0.11044544084478153697e-3_f64 * t10115 * t2024;
    (t94914, t94917, t94919, t94921, t94931)
}
