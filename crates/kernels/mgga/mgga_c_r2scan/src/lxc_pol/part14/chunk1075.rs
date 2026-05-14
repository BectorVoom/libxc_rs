//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1075/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1075<F: Float>(t37765: F, t37770: F, t37788: F, t39686: F, t39689: F, t39692: F, t39695: F, t39697: F, t39700: F, t39703: F, t39706: F, t39708: F, t39721: F, t39723: F, t37797: F, t37809: F, t37812: F, t37817: F, t39713: F, t39715: F, t39717: F, t39719: F, t39727: F, t39730: F) -> (F, F) {
    let t41511 = -0.51220160311720645768e0 * t37765 - 0.21951497276451705328e-1 * t37770 - 0.46230515946956099004e0 * t37788 + 0.13099107994629972538e-1 * t39686 - 0.87327386630866483588e-2 * t39689 - 0.26198215989259945076e-1 * t39692 + 0.1047928639570397803e0 * t39695 - 0.87327386630866483588e-2 * t39697 - 0.87327386630866483588e-2 * t39700 + 0.87327386630866483588e-2 * t39703 - 0.43663693315433241794e-2 * t39706 - 0.87327386630866483588e-2 * t39708;
    let t41518 = 0.57829097596741960691e-3 * t39721;
    let t41519 = 0.16262400898971305031e-3 * t39723;
    let t41526 = -0.26198215989259945076e-1 * t39713 + 0.87327386630866483588e-2 * t39715 - 0.26198215989259945076e-1 * t39717 - 0.2600466522016280569e0 * t39719 - t41518 + t41519 - 0.46230515946956099004e0 * t37797 + 0.26198215989259945076e-1 * t39727 + 0.87327386630866483588e-2 * t39730 + 0.54878743191129263322e-2 * t37809 + 0.23115257973478049502e0 * t37812 - 0.76830240467580968652e0 * t37817;
    (t41511, t41526)
}
