//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta757 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2835;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta757<F: Float>(t1063: F, t247: F, t42778: F, t906: F, t11986: F, t2858: F, t373: F, t675: F, t828: F, t3115: F, t3119: F, t11249: F, t3151: F, t3046: F, t3316: F, t4891: F, t11238: F, t196: F) -> (F, F, F, F, F, F, F, F) {
        let (t42781, t42785, t42792, t42793) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2835::<F>(t1063, t247, t42778, t906, t11986, t2858, t373, t675, t828);
        let (t42795, t42804, t42830, t42859) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2836::<F>(t3115, t3119, t42793, t11249, t3151, t3046, t3316, t4891, t11238, t196);
    (t42781, t42785, t42792, t42793, t42795, t42804, t42830, t42859)
}
