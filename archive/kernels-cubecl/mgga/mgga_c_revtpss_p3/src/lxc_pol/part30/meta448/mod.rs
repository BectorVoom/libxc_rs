//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1711;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta448<F: Float>(t3368: F, t5277: F, t1042: F, t3704: F, t5274: F, t1774: F, t3588: F, t1250: F, t3720: F, t1285: F, t17395: F, t1032: F, t5216: F, t1246: F, t1252: F, t12956: F, t12999: F, t13012: F, t13015: F, t13018: F, t3631: F, t3647: F, t3711: F, t3718: F, t5279: F, t5304: F) -> (F, F, F, F, F) {
        let (t17589, t17593, t17600, t17602, t17605, t17608) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1711::<F>(t3368, t5277, t1042, t3704, t5274, t1774, t3588, t1250, t3720, t1285, t17395, t1032, t5216);
        let t17614 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1712::<F>(t1246, t17608, t1252, t12956, t12999, t13012, t13015, t13018, t17589, t17593, t17602, t17605, t3631, t3647, t3711, t3718, t5279, t5304);
    (t17589, t17600, t17602, t17608, t17614)
}
