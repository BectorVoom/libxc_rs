//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1250/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1250<F: Float>(t10680: F, t11587: F, t40310: F, t10673: F, t11591: F, t40317: F, t37459: F, t37461: F, t37464: F, t37468: F, t37473: F, t37477: F, t40426: F, t40429: F, t40435: F, t40451: F, t40457: F, t40461: F, t42953: F) -> F {
    let t43875 = t10680 * t11587 * t40310;
    let t43878 = t10673 * t11591 * t40317;
    let t43883 = t42953 + F::new(0.72042316457491791906e-3) * t43875 - F::new(0.10248087766267884742e-3) * t43878 + t37459 - t37461 - t37464 + t40426 - t40429 + t40435 - F::new(0.43368970657079495312e-4) * t37468 - t37473 - F::new(0.35220688045884876043e-2) * t37477 - F::new(0.30487649791575028314e-3) * t40451 - t40457 + t40461;
    t43883
}
