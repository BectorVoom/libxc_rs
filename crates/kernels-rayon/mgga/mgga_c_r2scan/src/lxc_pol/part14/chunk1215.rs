//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1215/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1215(t39721: f64, t39723: f64, t37797: f64, t37809: f64, t37812: f64, t37817: f64, t39713: f64, t39715: f64, t39717: f64, t39719: f64, t39727: f64, t39730: f64) -> f64 {
    let t41518 = 0.57829097596741960691e-3_f64 * t39721;
    let t41519 = 0.16262400898971305031e-3_f64 * t39723;
    let t41526 = -0.26198215989259945076e-1_f64 * t39713 + 0.87327386630866483588e-2_f64 * t39715 - 0.26198215989259945076e-1_f64 * t39717 - 0.2600466522016280569e0_f64 * t39719 - t41518 + t41519 - 0.46230515946956099004e0_f64 * t37797 + 0.26198215989259945076e-1_f64 * t39727 + 0.87327386630866483588e-2_f64 * t39730 + 0.54878743191129263322e-2_f64 * t37809 + 0.23115257973478049502e0_f64 * t37812 - 0.76830240467580968652e0_f64 * t37817;
    t41526
}
