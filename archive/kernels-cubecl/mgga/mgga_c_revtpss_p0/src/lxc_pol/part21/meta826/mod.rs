//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta826 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3078;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta826<F: Float>(t12228: F, t1732: F, t44091: F, t44093: F, t43748: F, t5068: F, t45046: F, t5109: F, t12361: F, t16652: F, t12243: F, t16662: F, t1149: F, t16943: F, t3384: F, t16942: F, t3433: F, t3435: F, t56262: F, t56264: F, t56268: F, t56271: F) -> (F, F, F, F, F, F, F, F) {
        let (t56275, t56277, t56279, t56281, t56283) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3078::<F>(t12228, t1732, t44091, t44093, t43748, t5068, t45046, t5109, t12361, t16652, t12243, t16662);
        let (t56286, t56290, t56291) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3079::<F>(t1149, t16943, t3384, t16942, t3433, t3435, t56262, t56264, t56268, t56271, t56275, t56277, t56279, t56281, t56283);
    (t56275, t56277, t56279, t56281, t56283, t56286, t56290, t56291)
}
