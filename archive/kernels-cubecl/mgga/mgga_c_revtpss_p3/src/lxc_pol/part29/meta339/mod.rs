//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1258;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1259;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta339<F: Float>(t11704: F, t2251: F, t3109: F, t828: F, t3096: F, t3091: F, t1020: F, t3105: F, t247: F, t2862: F, t1063: F, t126: F, t3181: F, t2853: F, t1007: F, t3083: F, t1003: F, t3080: F, t221: F, t346: F, t68: F, t345: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11705, t11710, t11712, t11714, t11723, t11725) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1258::<F>(t11704, t2251, t3109, t828, t3096, t3091, t1020, t3105, t247, t2862, t1063, t126, t3181);
        let (t11728, t11730, t11732, t11737) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1259::<F>(t11725, t247, t2853, t1063, t1007, t3083, t1003, t3080, t221, t346, t68, t345);
    (t11705, t11710, t11712, t11714, t11723, t11728, t11730, t11732, t11737)
}
