//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 56/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk56<F: Float>(t15: F, t12: F, t56: F, t58: F, t60: F) -> (F, F, F, F, F, F, F) {
    let t148 = F::sqrt(F::new(4.0));
    let t149 = t148 * t15;
    let t151 = F::new(0.3138525e-1) * t12;
    let t152 = F::new(1.0) + F::new(0.22225e-1) * t149 + t151;
    let t153 = t152 * t152;
    let t154 = F::new(1.0) / t153;
    let t158 = F::new(1.0) - F::new(0.2363e1) * t58 * t56 * t60;
    (t148, t149, t151, t152, t153, t154, t158)
}
