//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1446;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1447;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta401<F: Float>(t1285: F, t17395: F, t1032: F, t5216: F, t1246: F, t12916: F, t5353: F, t3718: F, t5347: F, t1781: F, t697: F, t1222: F, t5284: F, t73: F, t17350: F, t3767: F, t372: F, t5277: F, t12865: F, t15904: F, t3623: F, t13148: F, t3172: F, t5303: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17605, t17609, t17619, t17622, t17629) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1446::<F>(t1285, t17395, t1032, t5216, t1246, t12916, t5353, t3718, t5347, t1781, t697, t1222);
        let (t17633, t17654, t17661, t17693, t17708, t17709, t17720) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1447::<F>(t5284, t73, t17350, t3767, t372, t5277, t1285, t12865, t15904, t3623, t13148, t3172, t5303);
    (t17605, t17609, t17619, t17622, t17629, t17633, t17654, t17661, t17693, t17708, t17709, t17720)
}
