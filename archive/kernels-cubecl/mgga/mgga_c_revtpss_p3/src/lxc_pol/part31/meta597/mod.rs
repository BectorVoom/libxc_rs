//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta597<F: Float>(t112: F, t843: F, t239: F, t655: F, t665: F, t2339: F, t624: F, t10208: F, t68: F, t25081: F, t7234: F, t116: F, t28159: F) -> (F, F, F, F, F, F, F) {
        let (t94974, t94975, t94976, t94978, t94982, t95088, t97622) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2027::<F>(t112, t843, t239, t655, t665, t2339, t624, t10208, t68, t25081, t7234, t116, t28159);
    (t94974, t94975, t94976, t94978, t94982, t95088, t97622)
}
