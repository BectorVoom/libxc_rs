//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk471;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk472;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta73<F: Float>(t2256: F, t30: F, t33: F, zeta_threshold: F, t36: F, t70: F, t607: F, t627: F, t362: F, t41: F, sigma0: F) -> (F, F, F, F, F, F, F) {
        let t2257 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk471::<F>(t2256);
        let t2258 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk472::<F>(t30, t33, t2257, zeta_threshold);
        let (t2259, t2260, t2263, t2269, t2270) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk473::<F>(t2258, t36, t70, t607, t627, t362, t41, sigma0);
    (t2257, t2258, t2259, t2260, t2263, t2269, t2270)
}
