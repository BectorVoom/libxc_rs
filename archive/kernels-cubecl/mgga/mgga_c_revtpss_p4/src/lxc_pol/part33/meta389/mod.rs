//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta389<F: Float>(t16712: F, t300: F, t5155: F, t16710: F, t16708: F, t1130: F, t5060: F, t1719: F, t3432: F, t5101: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16713, t16784, t16797, t16798, t16820, t16821, t16822, t16835, t16840, t16868, t16869, t16873) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1439::<F>(t16712, t300, t5155, t16710, t16708, t1130, t5060, t1719, t3432, t5101, t698);
    (t16713, t16784, t16797, t16798, t16820, t16821, t16822, t16835, t16840, t16868, t16869, t16873)
}
