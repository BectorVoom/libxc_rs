//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta834 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3122;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3123;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3124;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3125;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3126;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta834<F: Float>(t480: F, t57465: F, t1235: F, t1789: F, t2434: F, t371: F, t12987: F, t1803: F, t1261: F, t16746: F, t247: F, t3634: F, t1012: F, t44958: F, t13026: F, t140: F, t1222: F, t16715: F, t1224: F, t5052: F, t697: F, t1042: F, t12784: F, t12789: F, t12910: F, t12991: F, t17448: F, t17459: F, t17600: F, t17690: F, t17763: F, t3640: F, t3644: F, t3720: F, t5268: F, t5340: F, t5341: F, t53474: F, t54450: F, t56172: F, t56254: F, t57373: F, t57435: F, t57449: F, t57451: F, t57464: F, t1248: F, t16750: F, t12915: F, t17344: F, t17345: F, t1260: F, t44843: F, t17423: F, t17426: F, t11249: F, t5284: F, t343: F, t56: F, t816: F, t65: F, t12256: F, t1121: F, t1250: F, t12797: F, t12866: F, t13102: F, t17353: F, t17475: F, t17672: F, t17705: F, t17747: F, t17748: F, t3584: F, t3625: F, t3626: F, t3718: F, t3719: F, t44548: F, t44559: F, t44571: F, t44583: F, t5056: F, t51959: F, t5312: F, t5373: F, t5381: F, t56149: F, t56201: F, t56219: F, t56561: F, t606: F, t12772: F, t17634: F, t17395: F, t3746: F, t44586: F, t17689: F, t44425: F, t17435: F, t3667: F, t127: F, t17278: F, t1256: F, t17311: F, t17333: F, t12268: F, t29054: F, t12282: F, t12800: F, t12976: F, t13095: F, t16737: F, t17369: F, t17429: F, t17679: F, t17684: F, t17693: F, t17709: F, t17710: F, t17729: F, t17730: F, t17753: F, t17754: F, t1791: F, t20945: F, t21203: F, t3631: F, t3647: F, t44833: F, t5320: F, t5397: F) -> (F, F, F, F, F, F, F) {
        let (t57466, t57471, t57473, t57478) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3122::<F>(t480, t57465, t1235, t1789, t2434, t371, t12987, t1803, t1261, t16746, t247, t3634);
        let t57496 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3123::<F>(t1012, t44958, t13026, t140, t1222, t16715, t1224, t5052, t697, t1042, t1261, t12784, t12789, t12910, t12991, t17448, t17459, t17600, t17690, t17763, t3640, t3644, t3720, t5268, t5340, t5341, t53474, t54450, t56172, t56254, t57373, t57435, t57449, t57451, t57464, t57466, t57471, t57473, t57478);
        let (t57498, t57508, t57520, t57534, t57536) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3124::<F>(t1248, t16750, t12915, t17344, t17345, t247, t1260, t44843, t17423, t17426, t11249, t5284);
        let (t57548, t57555) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3125::<F>(t343, t56, t816, t13026, t65, t12256, t1121, t1222, t1250, t12797, t12866, t13102, t17353, t17426, t17475, t17672, t17705, t17747, t17748, t247, t3584, t3625, t3626, t3718, t3719, t3720, t44548, t44559, t44571, t44583, t5056, t51959, t5312, t5373, t5381, t56149, t56201, t56219, t56561, t57498, t57508, t57520, t57534, t57536, t606);
        let (t57569, t57571, t57578, t57584, t57586, t57590) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3126::<F>(t12772, t17634, t3625, t17395, t3746, t1248, t44586, t17689, t44425, t17435, t3667, t1235, t127, t17278, t371);
        let t57610 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3127::<F>(t1256, t17311, t17333, t12268, t29054, t12282, t1250, t12800, t12976, t13095, t16737, t17369, t17426, t17429, t17679, t17684, t17693, t17709, t17710, t17729, t17730, t17753, t17754, t1791, t20945, t21203, t3626, t3631, t3647, t3720, t44833, t51959, t5320, t5397, t57536, t57548, t57569, t57571, t57578, t57584, t57586, t57590);
    (t57496, t57498, t57536, t57548, t57555, t57578, t57610)
}
