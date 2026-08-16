//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta944 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3098;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3099;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3100;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3101;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3102;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta944(t81618: f64, t81621: f64, t81623: f64, t81625: f64, t81627: f64, t81629: f64, t81631: f64, t81633: f64, t81635: f64, t81638: f64, t81641: f64, t1149: f64, t24327: f64, t44017: f64, t24324: f64, t3384: f64, t24323: f64, t3435: f64, t3433: f64, t12227: f64, t20651: f64, t5104: f64, t24220: f64, t44091: f64, t44093: f64, t43771: f64, t45106: f64, t45107: f64, t68255: f64, t68257: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81399: f64, t81401: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64, t81416: f64, t68262: f64, t68277: f64, t68312: f64, t68332: f64, t68334: f64, t68336: f64, t68368: f64, t68370: f64, t81423: f64, t81425: f64, t81427: f64, t81429: f64, t58114: f64, t81439: f64, t81442: f64, t81445: f64, t81448: f64, t81451: f64, t81454: f64, t81457: f64, t81460: f64, t81463: f64, t81466: f64, t81469: f64, t56236: f64, t58117: f64, t58134: f64, t68389: f64, t68399: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t43888: f64, t58146: f64, t58153: f64, t58166: f64, t81242: f64, t81245: f64, t81489: f64, t81491: f64, t81494: f64, t81496: f64, t81499: f64, t81501: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81642, t81646) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3098(t81618, t81621, t81623, t81625, t81627, t81629, t81631, t81633, t81635, t81638, t81641, t1149, t24327, t44017);
        let (t81649, t81653, t81656, t81660) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3099(t1149, t24324, t3384, t24323, t3435, t3433, t12227, t20651, t5104, t24220, t44091, t44093);
        let (t81678, t81691) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3100(t43771, t45106, t45107, t68255, t68257, t81156, t81158, t81162, t81167, t81399, t81401, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214, t81416);
        let t81705 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3101(t68262, t68277, t68312, t68332, t68334, t68336, t68368, t68370, t81423, t81425, t81427, t81429);
        let t81717 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3102(t58114, t81439, t81442, t81445, t81448, t81451, t81454, t81457, t81460, t81463, t81466, t81469);
        let (t81729, t81740) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3103(t56236, t58117, t58134, t68389, t68399, t81224, t81228, t81230, t81232, t81234, t81236, t43888, t58146, t58153, t58166, t81242, t81245, t81489, t81491, t81494, t81496, t81499, t81501);
    (t81642, t81646, t81649, t81653, t81656, t81660, t81678, t81691, t81705, t81717, t81729, t81740)
}
