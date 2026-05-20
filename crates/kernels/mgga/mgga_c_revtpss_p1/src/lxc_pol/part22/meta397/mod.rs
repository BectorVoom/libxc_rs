//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1982;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1983;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1984;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta397<F: Float>(t13920: F, t543: F, t1390: F, t828: F, t1398: F, t1882: F, t3938: F, t13789: F, t13869: F, t13874: F, t1388: F, t13880: F, t1410: F, t3934: F, t9753: F, t9762: F, t9766: F, t9771: F, t9776: F, t9780: F, t9786: F, t9791: F, t4057: F, t5673: F, t5674: F, t13848: F, t9818: F, t9816: F, t125: F, t5658: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t13921 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1982::<F>(t13920, t543);
        let (t13923, t13926) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1983::<F>(t1390, t13921, t828, t1398, t1882);
        let (t13927, t13928, t13931) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1984::<F>(t13926, t3938, t13789, t13869, t13874, t1388, t13880, t13923, t1410, t3934, t9753, t9762, t9766, t9771, t9776, t9780, t9786, t9791);
        let (t13937, t13941, t13943, t13944) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1985::<F>(t4057, t5673, t5674, t13848, t3938, t9818, t9816, t125, t5658);
    (t13921, t13923, t13926, t13927, t13928, t13931, t13937, t13941, t13943, t13944)
}
