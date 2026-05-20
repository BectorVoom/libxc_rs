//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1693;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta352<F: Float>(t11735: F, t345: F, t10345: F, t344: F, t247: F, t2858: F, t3109: F, t1063: F, t1066: F, t11160: F, t1068: F, t11707: F, t11712: F, t11714: F, t11723: F, t11728: F, t11730: F, t11732: F, t3091: F, t3101: F, t3106: F, t3177: F, t3184: F, t348: F) -> (F, F, F, F, F, F) {
        let (t11737, t11738, t11744, t11745, t11748, t11751) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1693::<F>(t11735, t345, t10345, t344, t247, t2858, t3109, t1063, t1066, t11160, t1068, t11707, t11712, t11714, t11723, t11728, t11730, t11732, t3091, t3101, t3106, t3177, t3184, t348);
    (t11737, t11738, t11744, t11745, t11748, t11751)
}
