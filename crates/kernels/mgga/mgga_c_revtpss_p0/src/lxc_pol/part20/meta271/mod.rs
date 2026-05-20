//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1121;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta271<F: Float>(t247: F, t2862: F, t3109: F, t1063: F, t126: F, t3181: F, t2853: F, t1007: F, t3083: F, t1003: F, t3080: F, t221: F, t346: F, t68: F, t345: F, t10345: F, t344: F, t2858: F, t1066: F, t11160: F, t1068: F, t11707: F, t11712: F, t11714: F, t3091: F, t3101: F, t3106: F, t3177: F, t3184: F, t348: F) -> (F, F, F, F, F, F, F, F) {
        let (t11722, t11723, t11725, t11727, t11728, t11730, t11732, t11735) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1121::<F>(t247, t2862, t3109, t1063, t126, t3181, t2853, t1007, t3083, t1003, t3080, t221, t346, t68);
        let (t11738, t11744, t11748, t11751) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1122::<F>(t11735, t345, t10345, t344, t247, t2858, t3109, t1063, t1066, t11160, t1068, t11707, t11712, t11714, t11723, t11728, t11730, t11732, t3091, t3101, t3106, t3177, t3184, t348);
    (t11722, t11725, t11727, t11735, t11738, t11744, t11748, t11751)
}
