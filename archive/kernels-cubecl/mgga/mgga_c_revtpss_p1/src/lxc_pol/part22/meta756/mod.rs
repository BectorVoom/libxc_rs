//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta756 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2833;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta756<F: Float>(t1063: F, t11986: F, t247: F, t2862: F, t11880: F, t3241: F, t1011: F, t1016: F, t2438: F, t3237: F, t697: F, t1014: F, t11150: F, t1003: F, t11735: F, t221: F, t345: F, t346: F, t624: F, t3080: F, t3083: F, t11858: F, t16048: F, t1065: F, t215: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t42710, t42712, t42716, t42719, t42731) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2833::<F>(t1063, t11986, t247, t2862, t11880, t3241, t1011, t1016, t2438, t3237, t697, t1014, t11150);
        let (t42740, t42745, t42756, t42765, t42778) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2834::<F>(t1003, t11735, t221, t345, t346, t624, t3080, t3083, t11858, t16048, t1065, t215);
    (t42710, t42712, t42716, t42719, t42731, t42740, t42745, t42756, t42765, t42778)
}
