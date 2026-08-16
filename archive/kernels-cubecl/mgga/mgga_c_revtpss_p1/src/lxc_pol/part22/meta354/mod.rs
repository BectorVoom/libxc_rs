//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1853;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta354<F: Float>(t11986: F, t247: F, t906: F, t1063: F, t1062: F, t3196: F, t3223: F, t1052: F, t3147: F, t1036: F, t3141: F, t3229: F, t369: F, t361: F, t351: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11988, t11989, t11991, t11994) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1853::<F>(t11986, t247, t906, t1063, t1062, t3196, t3223);
        let (t11997, t11998, t11999, t12003, t12004) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1854::<F>(t1052, t3147, t1036, t3141, t3229, t369, t361, t351);
    (t11988, t11989, t11991, t11994, t11997, t11998, t11999, t12003, t12004)
}
