//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta936 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3076;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3077;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3078;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3079;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta936(t1196: f64, t20890: f64, t58647: f64, t24473: f64, t3531: f64, t24764: f64, t5206: f64, t20400: f64, t5207: f64, t20692: f64, t29322: f64, t5023: f64, t5501: f64, t73252: f64, t81322: f64, t81326: f64, t81328: f64, t81330: f64, t81333: f64, t24324: f64, t3379: f64, t43881: f64, t68255: f64, t68257: f64, t68262: f64, t68277: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64, t43888: f64, t56236: f64, t58073: f64, t58075: f64, t58090: f64, t68332: f64, t68334: f64, t68336: f64, t68389: f64, t68399: f64, t68454: f64, t68456: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t81242: f64, t81245: f64, t1132: f64, t1139: f64, t43771: f64, t44039: f64, t44040: f64, t44348: f64, t52011: f64, t77513: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81336, t81338, t81341, t81343, t81350) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3076(t1196, t20890, t58647, t24473, t3531, t24764, t5206, t20400, t5207, t20692, t29322, t5023, t5501, t73252, t81322, t81326, t81328, t81330, t81333);
        let (t81352, t81379) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3077(t24324, t3379, t43881, t68255, t68257, t68262, t68277, t81156, t81158, t81162, t81167, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
        let t81397 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3078(t43888, t56236, t58073, t58075, t58090, t68332, t68334, t68336, t68389, t68399, t68454, t68456, t81224, t81228, t81230, t81232, t81234, t81236, t81242, t81245);
        let (t81399, t81401, t81403) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3079(t81379, t81397, t1132, t1139, t43771, t44039, t44040, t68255, t68257, t81156, t81158, t81162, t81167);
        let (t81416, t81418) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3080(t44348, t52011, t77513, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
    (t81336, t81338, t81341, t81343, t81350, t81352, t81399, t81401, t81403, t81416, t81418)
}
