//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1109/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1109<F: Float>(t40460: F, t37483: F, t37488: F, t37495: F, t39083: F, t40451: F, t40485: F, t41806: F, t41808: F, t41811: F, t41814: F, t41818: F, t41821: F, t41824: F, t42215: F, t40518: F) -> (F, F) {
    let t42216 = 0.86737941314158990616e-4 * t40460;
    let t42221 = -0.30487649791575028312e-3 * t40451 - t42215 + t42216 + t41806 + t41808 + t39083 - t41811 + t41814 - 0.78064147182743091556e-3 * t37483 + t41818 + 0.29810146462873361016e-2 * t40485 + t41821 + 0.72042316457491791901e-3 * t37488 + 0.1440846329149835838e-2 * t37495 + t41824;
    let t42229 = 0.60975299583150056624e-3 * t40518;
    (t42221, t42229)
}
