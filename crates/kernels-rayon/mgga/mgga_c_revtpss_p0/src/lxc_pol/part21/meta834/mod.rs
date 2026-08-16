//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta834 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3122;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3123;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3124;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3125;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3126;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta834(t480: f64, t57465: f64, t1235: f64, t1789: f64, t2434: f64, t371: f64, t12987: f64, t1803: f64, t1261: f64, t16746: f64, t247: f64, t3634: f64, t1012: f64, t44958: f64, t13026: f64, t140: f64, t1222: f64, t16715: f64, t1224: f64, t5052: f64, t697: f64, t1042: f64, t12784: f64, t12789: f64, t12910: f64, t12991: f64, t17448: f64, t17459: f64, t17600: f64, t17690: f64, t17763: f64, t3640: f64, t3644: f64, t3720: f64, t5268: f64, t5340: f64, t5341: f64, t53474: f64, t54450: f64, t56172: f64, t56254: f64, t57373: f64, t57435: f64, t57449: f64, t57451: f64, t57464: f64, t1248: f64, t16750: f64, t12915: f64, t17344: f64, t17345: f64, t1260: f64, t44843: f64, t17423: f64, t17426: f64, t11249: f64, t5284: f64, t343: f64, t56: f64, t816: f64, t65: f64, t12256: f64, t1121: f64, t1250: f64, t12797: f64, t12866: f64, t13102: f64, t17353: f64, t17475: f64, t17672: f64, t17705: f64, t17747: f64, t17748: f64, t3584: f64, t3625: f64, t3626: f64, t3718: f64, t3719: f64, t44548: f64, t44559: f64, t44571: f64, t44583: f64, t5056: f64, t51959: f64, t5312: f64, t5373: f64, t5381: f64, t56149: f64, t56201: f64, t56219: f64, t56561: f64, t606: f64, t12772: f64, t17634: f64, t17395: f64, t3746: f64, t44586: f64, t17689: f64, t44425: f64, t17435: f64, t3667: f64, t127: f64, t17278: f64, t1256: f64, t17311: f64, t17333: f64, t12268: f64, t29054: f64, t12282: f64, t12800: f64, t12976: f64, t13095: f64, t16737: f64, t17369: f64, t17429: f64, t17679: f64, t17684: f64, t17693: f64, t17709: f64, t17710: f64, t17729: f64, t17730: f64, t17753: f64, t17754: f64, t1791: f64, t20945: f64, t21203: f64, t3631: f64, t3647: f64, t44833: f64, t5320: f64, t5397: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t57466, t57471, t57473, t57478) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3122(t480, t57465, t1235, t1789, t2434, t371, t12987, t1803, t1261, t16746, t247, t3634);
        let t57496 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3123(t1012, t44958, t13026, t140, t1222, t16715, t1224, t5052, t697, t1042, t1261, t12784, t12789, t12910, t12991, t17448, t17459, t17600, t17690, t17763, t3640, t3644, t3720, t5268, t5340, t5341, t53474, t54450, t56172, t56254, t57373, t57435, t57449, t57451, t57464, t57466, t57471, t57473, t57478);
        let (t57498, t57508, t57520, t57534, t57536) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3124(t1248, t16750, t12915, t17344, t17345, t247, t1260, t44843, t17423, t17426, t11249, t5284);
        let (t57548, t57555) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3125(t343, t56, t816, t13026, t65, t12256, t1121, t1222, t1250, t12797, t12866, t13102, t17353, t17426, t17475, t17672, t17705, t17747, t17748, t247, t3584, t3625, t3626, t3718, t3719, t3720, t44548, t44559, t44571, t44583, t5056, t51959, t5312, t5373, t5381, t56149, t56201, t56219, t56561, t57498, t57508, t57520, t57534, t57536, t606);
        let (t57569, t57571, t57578, t57584, t57586, t57590) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3126(t12772, t17634, t3625, t17395, t3746, t1248, t44586, t17689, t44425, t17435, t3667, t1235, t127, t17278, t371);
        let t57610 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3127(t1256, t17311, t17333, t12268, t29054, t12282, t1250, t12800, t12976, t13095, t16737, t17369, t17426, t17429, t17679, t17684, t17693, t17709, t17710, t17729, t17730, t17753, t17754, t1791, t20945, t21203, t3626, t3631, t3647, t3720, t44833, t51959, t5320, t5397, t57536, t57548, t57569, t57571, t57578, t57584, t57586, t57590);
    (t57496, t57498, t57536, t57548, t57555, t57578, t57610)
}
