//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta944 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3098;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3099;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3100;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3101;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3102;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta944<F: Float>(t81618: F, t81621: F, t81623: F, t81625: F, t81627: F, t81629: F, t81631: F, t81633: F, t81635: F, t81638: F, t81641: F, t1149: F, t24327: F, t44017: F, t24324: F, t3384: F, t24323: F, t3435: F, t3433: F, t12227: F, t20651: F, t5104: F, t24220: F, t44091: F, t44093: F, t43771: F, t45106: F, t45107: F, t68255: F, t68257: F, t81156: F, t81158: F, t81162: F, t81167: F, t81399: F, t81401: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F, t81416: F, t68262: F, t68277: F, t68312: F, t68332: F, t68334: F, t68336: F, t68368: F, t68370: F, t81423: F, t81425: F, t81427: F, t81429: F, t58114: F, t81439: F, t81442: F, t81445: F, t81448: F, t81451: F, t81454: F, t81457: F, t81460: F, t81463: F, t81466: F, t81469: F, t56236: F, t58117: F, t58134: F, t68389: F, t68399: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t43888: F, t58146: F, t58153: F, t58166: F, t81242: F, t81245: F, t81489: F, t81491: F, t81494: F, t81496: F, t81499: F, t81501: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81642, t81646) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3098::<F>(t81618, t81621, t81623, t81625, t81627, t81629, t81631, t81633, t81635, t81638, t81641, t1149, t24327, t44017);
        let (t81649, t81653, t81656, t81660) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3099::<F>(t1149, t24324, t3384, t24323, t3435, t3433, t12227, t20651, t5104, t24220, t44091, t44093);
        let (t81678, t81691) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3100::<F>(t43771, t45106, t45107, t68255, t68257, t81156, t81158, t81162, t81167, t81399, t81401, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214, t81416);
        let t81705 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3101::<F>(t68262, t68277, t68312, t68332, t68334, t68336, t68368, t68370, t81423, t81425, t81427, t81429);
        let t81717 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3102::<F>(t58114, t81439, t81442, t81445, t81448, t81451, t81454, t81457, t81460, t81463, t81466, t81469);
        let (t81729, t81740) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3103::<F>(t56236, t58117, t58134, t68389, t68399, t81224, t81228, t81230, t81232, t81234, t81236, t43888, t58146, t58153, t58166, t81242, t81245, t81489, t81491, t81494, t81496, t81499, t81501);
    (t81642, t81646, t81649, t81653, t81656, t81660, t81678, t81691, t81705, t81717, t81729, t81740)
}
