//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1144/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1144(t11780: f64, t2207: f64, t3328: f64, t11793: f64, t2201: f64, t3336: f64, t37797: f64, t37809: f64, t37812: f64, t37817: f64, t39713: f64, t39715: f64, t39717: f64, t39719: f64, t39721: f64, t39723: f64) -> f64 {
    let t39727 = t2207 * t11780 * t3328;
    let t39730 = t2201 * t3336 * t11793;
    let t39735 = -0.13099107994629972538e-1_f64 * t39713 + 0.43663693315433241792e-2_f64 * t39715 - 0.13099107994629972538e-1_f64 * t39717 - 0.13002332610081402845e0_f64 * t39719 - 0.28914548798370980346e-3_f64 * t39721 + 0.81312004494856525156e-4_f64 * t39723 - 0.23115257973478049502e0_f64 * t37797 + 0.13099107994629972538e-1_f64 * t39727 + 0.43663693315433241792e-2_f64 * t39730 + 0.27439371595564631661e-2_f64 * t37809 + 0.11557628986739024751e0_f64 * t37812 - 0.38415120233790484326e0_f64 * t37817;
    t39735
}
