//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2174;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2175;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2176;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2177;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta665(t27888: f64, t27899: f64, t27884: f64, t27873: f64, t97700: f64, t98041: f64, t22387: f64, t22415: f64, t28012: f64, t7279: f64, t7917: f64, t94851: f64, t94854: f64, t94857: f64, t98043: f64, t98069: f64, t98071: f64, t98078: f64, t98081: f64, t1444: f64, t6874: f64, t22453: f64, t94901: f64, t108368: f64, t25895: f64, t108225: f64, t14230: f64, t25930: f64, t25931: f64, t27868: f64, t27973: f64, t27981: f64, t3999: f64, t6918: f64, t7274: f64, t7295: f64, t7296: f64, t75012: f64, t7910: f64, t94865: f64, t94867: f64, t97933: f64, t98084: f64, t98089: f64, t98091: f64, t98099: f64, t108187: f64, t25878: f64, t6861: f64, t30081: f64, t689: f64, t94768: f64, t94763: f64, t5722: f64, t97783: f64, t2022: f64, t22252: f64, t25921: f64, t30057: f64, t30089: f64, t543: f64, t7292: f64, t7301: f64, t94876: f64, t98101: f64, t98104: f64, t98305: f64, t98310: f64, t98312: f64, t98314: f64, t6862: f64, t22107: f64, t26028: f64, t22111: f64, t22271: f64, t27940: f64, t22163: f64, t6871: f64, t94429: f64, t22159: f64, t98115: f64, t22120: f64, t22076: f64, t22102: f64, t94423: f64, t22081: f64, t22085: f64, t98108: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t108443 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2174(t27888, t27899, t27884, t27873, t97700, t98041, t22387, t22415, t28012, t7279, t7917, t94851, t94854, t94857, t98043, t98069, t98071, t98078, t98081);
        let t108471 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2175(t1444, t6874, t22453, t94901, t108368, t25895, t108225, t14230, t25930, t25931, t27868, t27973, t27981, t3999, t6918, t7274, t7295, t7296, t75012, t7910, t94865, t94867, t97933, t98084, t98089, t98091, t98099);
        let (t108484, t108500) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2176(t108187, t25878, t6861, t7274, t30081, t689, t94768, t94763, t5722, t97783, t2022, t22252, t25921, t30057, t30089, t543, t7292, t7295, t7301, t94876, t98101, t98104, t98305, t98310, t98312, t98314);
        let (t108502, t108508, t108510, t108512, t108514, t108516, t108518) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2177(t1444, t6862, t22107, t26028, t22111, t22271, t27940, t22163, t6871, t94429, t22159, t98115);
        let t108530 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2178(t22120, t26028, t22076, t22102, t94423, t22081, t22085, t108508, t108510, t108512, t108514, t108516, t108518, t98108);
    (t108443, t108471, t108484, t108500, t108502, t108530)
}
