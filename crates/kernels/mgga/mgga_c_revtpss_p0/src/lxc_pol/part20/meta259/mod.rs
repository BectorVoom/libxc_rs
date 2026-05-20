//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1097;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1098;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1099;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1100;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1101;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta259<F: Float>(t11467: F, t3014: F, t11132: F, t11337: F, t11158: F, t11162: F, t11167: F, t11316: F, t11319: F, t11322: F, t11326: F, t11329: F, t11332: F, t11339: F, t11343: F, t11346: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11171: F, t11356: F, t11359: F, t11366: F, t11368: F, t11370: F, t11373: F, t11376: F, t973: F, t3010: F, t963: F, t315: F, t3013: F, t323: F, t2962: F, t955: F, t2970: F, t953: F, t11114: F, t11118: F, t11399: F, t11404: F, t11409: F, t11411: F, t11445: F, t11450: F, t11453: F, t11456: F, t11461: F, t11466: F, t2938: F, t2943: F, t2963: F, t2968: F, t2971: F, t2982: F, t3007: F, t3015: F, t946: F, t965: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11468, t11485) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1097::<F>(t11467, t3014, t11132, t11337, t11158, t11162, t11167, t11316, t11319, t11322, t11326, t11329, t11332, t11339, t11343, t11346);
        let t11500 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1098::<F>(t11134, t11136, t11138, t11140, t11147, t11153, t11171, t11356, t11359, t11366, t11368, t11370, t11373, t11376);
        let t11501 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1099::<F>(t11485, t11500);
        let (t11502, t11506) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1100::<F>(t11501, t973, t3010, t963);
        let (t11507, t11509) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1101::<F>(t11506, t315, t3013, t323);
        let (t11510, t11513, t11517, t11520) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1102::<F>(t11467, t11509, t2962, t955, t2970, t953, t11114, t11118, t11399, t11404, t11409, t11411, t11445, t11450, t11453, t11456, t11461, t11466, t11468, t11502, t11507, t2938, t2943, t2963, t2968, t2971, t2982, t3007, t3015, t946, t965, t974);
    (t11468, t11501, t11502, t11506, t11507, t11509, t11510, t11513, t11517, t11520)
}
