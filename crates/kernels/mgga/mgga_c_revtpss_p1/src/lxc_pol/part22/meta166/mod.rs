//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta166 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1107;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1108;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1109;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1110;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta166<F: Float>(t1390: F, t3924: F, t828: F, t1386: F, t820: F, t843: F, t1401: F, t241: F, t1412: F, t72: F, t245: F, t125: F, t1398: F, t1353: F, t543: F, t159: F, t550: F, t216: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3926, t3930) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1107::<F>(t1390, t3924, t828, t1386, t820, t843);
        let (t3931, t3934) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1108::<F>(t1401, t3930, t1386, t241, t820);
        let (t3935, t3936) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1109::<F>(t1412, t72, t245);
        let (t3937, t3938) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1110::<F>(t125, t1398, t1353, t543);
        let (t3940, t3943, t3944) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1111::<F>(t3937, t3938, t3936, t159, t550, t216);
    (t3926, t3930, t3931, t3934, t3935, t3936, t3938, t3940, t3943, t3944)
}
