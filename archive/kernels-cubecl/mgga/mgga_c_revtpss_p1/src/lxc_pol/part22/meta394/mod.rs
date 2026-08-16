//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1970;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1971;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1972;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta394<F: Float>(t4004: F, t5673: F, t5674: F, t9840: F, t1868: F, t3829: F, t828: F, t9942: F, t5608: F, t5675: F, t9934: F, t2661: F, t3936: F, t5704: F, t3924: F, t2482: F, t4000: F, t814: F, t136: F, t550: F, t220: F, t124: F, t1882: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13817, t13821, t13824, t13826, t13830, t13832) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1970::<F>(t4004, t5673, t5674, t9840, t1868, t3829, t828, t9942, t5608, t5675, t9934, t2661);
        let (t13834, t13841, t13845) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1971::<F>(t3936, t4004, t5704, t3924, t2482, t4000, t814);
        let t13847 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1972::<F>(t136, t550, t220);
        let t13848 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1973::<F>(t124, t1882);
    (t13817, t13821, t13824, t13826, t13830, t13832, t13834, t13841, t13845, t13847, t13848)
}
