//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 639/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk639<F: Float>(t3450: F, t3455: F, t3459: F, t3468: F, t3471: F, t3475: F, t3496: F, t3499: F, t3564: F, t3565: F, t3566: F, t797: F, t910: F) -> (F, F, F) {
    let t3567 = F::new(0.30487649791575028312e-3) * t3450;
    let t3570 = -t3564 + t3565 - t3566 - t3567 - F::new(0.72042316457491791901e-3) * t3455 + F::new(0.30487649791575028312e-3) * t3459 - t3468 - t3471 + t3475 - t3496 + t3499;
    let t3574 = t797 * t910;
    (t3567, t3570, t3574)
}
