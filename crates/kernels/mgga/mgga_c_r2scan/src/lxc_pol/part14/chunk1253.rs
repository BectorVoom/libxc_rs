//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1253/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1253<F: Float>(t39260: F, t37377: F, t37380: F, t39054: F, t41192: F, t41196: F, t41199: F, t41201: F, t41205: F, t41208: F, t41211: F, t41213: F, t41216: F, t41219: F, t41221: F) -> F {
    let t42170 = F::new(0.162600798888400151e-2) * t39260;
    let t42171 = F::new(0.1921128438866447784e-2) * t37377 - F::new(0.81300399444200075499e-3) * t37380 + t41192 - t41196 - t41199 - t41201 - t41205 - t41208 - t39054 + t41211 - t41213 + t41216 - t42170 + t41219 - t41221;
    t42171
}
