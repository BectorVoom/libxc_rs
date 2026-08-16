//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta330<F: Float>(t3727: F, t460: F, t12295: F, t1284: F, t3552: F, t1204: F, t3766: F, t3555: F, t3754: F, t1248: F, t3153: F, t3588: F, t5464: F) -> (F, F, F, F, F, F, F) {
        let (t12673, t12678, t12699, t12702, t12709, t12712, t12713) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1110::<F>(t3727, t460, t12295, t1284, t3552, t1204, t3766, t3555, t3754, t1248, t3153, t3588, t5464);
    (t12673, t12678, t12699, t12702, t12709, t12712, t12713)
}
