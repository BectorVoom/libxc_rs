//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta362<F: Float>(t12295: F, t1284: F, t3552: F, t1204: F, t3766: F, t3555: F, t3754: F, t1248: F, t3153: F, t3588: F, t5464: F, t3566: F) -> (F, F, F, F, F, F, F) {
        let (t12678, t12699, t12702, t12709, t12712, t12713, t12717) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1388::<F>(t12295, t1284, t3552, t1204, t3766, t3555, t3754, t1248, t3153, t3588, t5464, t3566);
    (t12678, t12699, t12702, t12709, t12712, t12713, t12717)
}
