//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1453;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta402<F: Float>(t1285: F, t17395: F, t1032: F, t5216: F, t1246: F, t12916: F, t5353: F, t3718: F, t5347: F, t1781: F, t697: F, t1222: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17605, t17608, t17609, t17617, t17619, t17620, t17622, t17628, t17629) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1453::<F>(t1285, t17395, t1032, t5216, t1246, t12916, t5353, t3718, t5347, t1781, t697, t1222);
    (t17605, t17608, t17609, t17617, t17619, t17620, t17622, t17628, t17629)
}
