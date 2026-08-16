//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1144/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1144<F: Float>(t11780: F, t2207: F, t3328: F, t11793: F, t2201: F, t3336: F, t37797: F, t37809: F, t37812: F, t37817: F, t39713: F, t39715: F, t39717: F, t39719: F, t39721: F, t39723: F) -> F {
    let t39727 = t2207 * t11780 * t3328;
    let t39730 = t2201 * t3336 * t11793;
    let t39735 = -F::cast_from(0.13099107994629972538e-1_f64) * t39713 + F::cast_from(0.43663693315433241792e-2_f64) * t39715 - F::cast_from(0.13099107994629972538e-1_f64) * t39717 - F::cast_from(0.13002332610081402845e0_f64) * t39719 - F::cast_from(0.28914548798370980346e-3_f64) * t39721 + F::cast_from(0.81312004494856525156e-4_f64) * t39723 - F::cast_from(0.23115257973478049502e0_f64) * t37797 + F::cast_from(0.13099107994629972538e-1_f64) * t39727 + F::cast_from(0.43663693315433241792e-2_f64) * t39730 + F::cast_from(0.27439371595564631661e-2_f64) * t37809 + F::cast_from(0.11557628986739024751e0_f64) * t37812 - F::cast_from(0.38415120233790484326e0_f64) * t37817;
    t39735
}
