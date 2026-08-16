//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta218 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk965;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta218<F: Float>(t2852: F, t357: F, t3109: F, t828: F, t126: F, t3181: F, t221: F, t346: F, t68: F, t345: F, t1014: F, t245: F, t3089: F, t3088: F, t3114: F) -> (F, F, F, F, F, F, F, F) {
        let (t11704, t11710) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk965::<F>(t2852, t357, t3109, t828);
        let (t11725, t11737, t11765, t11772, t11773, t11774) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk966::<F>(t126, t3181, t221, t346, t68, t345, t1014, t2852, t245, t3089, t3088, t3114);
    (t11704, t11710, t11725, t11737, t11765, t11772, t11773, t11774)
}
