//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1092/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1092<F: Float>(t10609: F, t1561: F, t97: F, t11584: F, t37365: F, t10673: F, t11587: F, t37360: F, t37373: F, t37426: F, t37427: F, t37428: F, t898: F) -> (F, F, F, F, F) {
    let t39197 = t97 * t10609 * t1561;
    let t39215 = t37365 * t11584;
    let t39218 = t10673 * t11587 * t37360;
    let t39221 = t37373 * t11584;
    let t39225 = t37426 * t37427 * t898 * t37428;
    (t39197, t39215, t39218, t39221, t39225)
}
