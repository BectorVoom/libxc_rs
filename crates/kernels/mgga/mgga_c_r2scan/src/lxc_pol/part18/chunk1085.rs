//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1085/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1085<F: Float>(t11557: F, t3261: F, t5086: F, t97: F, t10609: F, t1561: F, t13908: F, t986: F, t3270: F, t11584: F, t37365: F, t10673: F, t11587: F, t37360: F) -> (F, F, F, F, F, F) {
    let t39177 = F::new(5.0) / F::new(8.0) * t11557;
    let t39190 = t97 * t3261 * t5086;
    let t39197 = t97 * t10609 * t1561;
    let t39202 = t13908 * t986;
    let t39203 = t3270 * t39202;
    let t39215 = t37365 * t11584;
    let t39218 = t10673 * t11587 * t37360;
    (t39177, t39190, t39197, t39203, t39215, t39218)
}
