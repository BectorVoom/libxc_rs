//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2052/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2052(t10073: f64, t25920: f64, t25938: f64, t25898: f64, t94889: f64, t25901: f64, t10115: f64, t2024: f64, t112: f64, t843: f64, t239: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94919 = t10073 * t25920 * t25938;
    let t94921 = t94889 * t25898;
    let t94922 = t94921 * t25901;
    let t94931 = 0.11044544084478153697e-3_f64 * t10115 * t2024;
    let t94973 = t843 * t112;
    let t94974 = 154.0_f64 / 27.0_f64 * t94973;
    let t94975 = t239 * t655;
    (t94919, t94921, t94922, t94931, t94974, t94975)
}
