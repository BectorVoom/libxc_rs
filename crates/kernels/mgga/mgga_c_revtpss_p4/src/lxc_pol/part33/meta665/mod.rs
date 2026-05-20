//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta665 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2174;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2175;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2176;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2177;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta665<F: Float>(t27888: F, t27899: F, t27884: F, t27873: F, t97700: F, t98041: F, t22387: F, t22415: F, t28012: F, t7279: F, t7917: F, t94851: F, t94854: F, t94857: F, t98043: F, t98069: F, t98071: F, t98078: F, t98081: F, t1444: F, t6874: F, t22453: F, t94901: F, t108368: F, t25895: F, t108225: F, t14230: F, t25930: F, t25931: F, t27868: F, t27973: F, t27981: F, t3999: F, t6918: F, t7274: F, t7295: F, t7296: F, t75012: F, t7910: F, t94865: F, t94867: F, t97933: F, t98084: F, t98089: F, t98091: F, t98099: F, t108187: F, t25878: F, t6861: F, t30081: F, t689: F, t94768: F, t94763: F, t5722: F, t97783: F, t2022: F, t22252: F, t25921: F, t30057: F, t30089: F, t543: F, t7292: F, t7301: F, t94876: F, t98101: F, t98104: F, t98305: F, t98310: F, t98312: F, t98314: F, t6862: F, t22107: F, t26028: F, t22111: F, t22271: F, t27940: F, t22163: F, t6871: F, t94429: F, t22159: F, t98115: F, t22120: F, t22076: F, t22102: F, t94423: F, t22081: F, t22085: F, t98108: F) -> (F, F, F, F, F, F) {
        let t108443 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2174::<F>(t27888, t27899, t27884, t27873, t97700, t98041, t22387, t22415, t28012, t7279, t7917, t94851, t94854, t94857, t98043, t98069, t98071, t98078, t98081);
        let t108471 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2175::<F>(t1444, t6874, t22453, t94901, t108368, t25895, t108225, t14230, t25930, t25931, t27868, t27973, t27981, t3999, t6918, t7274, t7295, t7296, t75012, t7910, t94865, t94867, t97933, t98084, t98089, t98091, t98099);
        let (t108484, t108500) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2176::<F>(t108187, t25878, t6861, t7274, t30081, t689, t94768, t94763, t5722, t97783, t2022, t22252, t25921, t30057, t30089, t543, t7292, t7295, t7301, t94876, t98101, t98104, t98305, t98310, t98312, t98314);
        let (t108502, t108508, t108510, t108512, t108514, t108516, t108518) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2177::<F>(t1444, t6862, t22107, t26028, t22111, t22271, t27940, t22163, t6871, t94429, t22159, t98115);
        let t108530 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2178::<F>(t22120, t26028, t22076, t22102, t94423, t22081, t22085, t108508, t108510, t108512, t108514, t108516, t108518, t98108);
    (t108443, t108471, t108484, t108500, t108502, t108530)
}
