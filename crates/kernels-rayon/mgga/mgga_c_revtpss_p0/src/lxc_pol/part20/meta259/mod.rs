//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1097;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1098;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1099;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1100;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1101;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta259(t11467: f64, t3014: f64, t11132: f64, t11337: f64, t11158: f64, t11162: f64, t11167: f64, t11316: f64, t11319: f64, t11322: f64, t11326: f64, t11329: f64, t11332: f64, t11339: f64, t11343: f64, t11346: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11171: f64, t11356: f64, t11359: f64, t11366: f64, t11368: f64, t11370: f64, t11373: f64, t11376: f64, t973: f64, t3010: f64, t963: f64, t315: f64, t3013: f64, t323: f64, t2962: f64, t955: f64, t2970: f64, t953: f64, t11114: f64, t11118: f64, t11399: f64, t11404: f64, t11409: f64, t11411: f64, t11445: f64, t11450: f64, t11453: f64, t11456: f64, t11461: f64, t11466: f64, t2938: f64, t2943: f64, t2963: f64, t2968: f64, t2971: f64, t2982: f64, t3007: f64, t3015: f64, t946: f64, t965: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11468, t11485) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1097(t11467, t3014, t11132, t11337, t11158, t11162, t11167, t11316, t11319, t11322, t11326, t11329, t11332, t11339, t11343, t11346);
        let t11500 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1098(t11134, t11136, t11138, t11140, t11147, t11153, t11171, t11356, t11359, t11366, t11368, t11370, t11373, t11376);
        let t11501 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1099(t11485, t11500);
        let (t11502, t11506) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1100(t11501, t973, t3010, t963);
        let (t11507, t11509) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1101(t11506, t315, t3013, t323);
        let (t11510, t11513, t11517, t11520) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1102(t11467, t11509, t2962, t955, t2970, t953, t11114, t11118, t11399, t11404, t11409, t11411, t11445, t11450, t11453, t11456, t11461, t11466, t11468, t11502, t11507, t2938, t2943, t2963, t2968, t2971, t2982, t3007, t3015, t946, t965, t974);
    (t11468, t11501, t11502, t11506, t11507, t11509, t11510, t11513, t11517, t11520)
}
