//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 975/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk975<F: Float>(t38341: F, t38346: F, t38349: F, t38362: F, t3261: F, t5086: F, t97: F, t10609: F, t1561: F, t11584: F, t37365: F, t10673: F, t11587: F, t37360: F, t37373: F, t37426: F, t37427: F, t37428: F, t898: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t39129 = 0.2439011983326002265e-2 * t38341;
    let t39130 = 0.18292589874945016987e-2 * t38346;
    let t39131 = 0.30487649791575028312e-3 * t38349;
    let t39134 = 0.91462949374725084936e-3 * t38362;
    let t39190 = t97 * t3261 * t5086;
    let t39197 = t97 * t10609 * t1561;
    let t39215 = t37365 * t11584;
    let t39218 = t10673 * t11587 * t37360;
    let t39221 = t37373 * t11584;
    let t39225 = t37426 * t37427 * t898 * t37428;
    (t39129, t39130, t39131, t39134, t39190, t39197, t39215, t39218, t39221, t39225)
}
