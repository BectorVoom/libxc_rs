//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1215/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1215<F: Float>(t39721: F, t39723: F, t37797: F, t37809: F, t37812: F, t37817: F, t39713: F, t39715: F, t39717: F, t39719: F, t39727: F, t39730: F) -> F {
    let t41518 = F::new(0.57829097596741960691e-3) * t39721;
    let t41519 = F::new(0.16262400898971305031e-3) * t39723;
    let t41526 = -F::new(0.26198215989259945076e-1) * t39713 + F::new(0.87327386630866483588e-2) * t39715 - F::new(0.26198215989259945076e-1) * t39717 - F::new(0.2600466522016280569e0) * t39719 - t41518 + t41519 - F::new(0.46230515946956099004e0) * t37797 + F::new(0.26198215989259945076e-1) * t39727 + F::new(0.87327386630866483588e-2) * t39730 + F::new(0.54878743191129263322e-2) * t37809 + F::new(0.23115257973478049502e0) * t37812 - F::new(0.76830240467580968652e0) * t37817;
    t41526
}
