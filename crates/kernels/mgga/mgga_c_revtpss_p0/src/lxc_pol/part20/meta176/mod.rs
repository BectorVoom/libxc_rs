//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta176 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk916;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta176<F: Float>(t30: F, t1448: F, t4144: F, t4146: F, t565: F, t1333: F, t3860: F, t4147: F, t513: F, t3874: F, t605: F, t1344: F, t2257: F, t9336: F, t9344: F, zeta_threshold: F, t33: F, t516: F, t1113: F, t3881: F, t1348: F, t3351: F, t9351: F, t9357: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9590, t9593, t9598, t9599, t9603, t9605, t9608, t9614) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk916::<F>(t30, t1448, t4144, t4146, t565, t1333, t3860, t4147, t513, t3874, t605, t1344, t2257, t9336, t9344, zeta_threshold);
        let (t9615, t9617, t9620, t9628) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk917::<F>(t33, t516, t1113, t3881, t1348, t3351, t9351, t9357, t9614, zeta_threshold);
    (t9590, t9593, t9598, t9599, t9603, t9605, t9608, t9615, t9617, t9620, t9628)
}
