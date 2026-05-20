//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1440;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta410<F: Float>(t9422: F, t9559: F, t9566: F, t9570: F, t9578: F, t13643: F, t9421: F, t9427: F, t9429: F, t9514: F, t9517: F, t9521: F, t9546: F, t9569: F, t9574: F, t9577: F) -> (F, F, F, F, F, F) {
        let (t22205, t22206, t22207, t22208, t22209, t22210) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1440::<F>(t9422, t9559, t9566, t9570, t9578, t13643, t9421, t9427, t9429, t9514, t9517, t9521, t9546, t9569, t9574, t9577);
    (t22205, t22206, t22207, t22208, t22209, t22210)
}
