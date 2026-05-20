//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta936 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3076;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3077;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3078;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3079;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta936<F: Float>(t1196: F, t20890: F, t58647: F, t24473: F, t3531: F, t24764: F, t5206: F, t20400: F, t5207: F, t20692: F, t29322: F, t5023: F, t5501: F, t73252: F, t81322: F, t81326: F, t81328: F, t81330: F, t81333: F, t24324: F, t3379: F, t43881: F, t68255: F, t68257: F, t68262: F, t68277: F, t81156: F, t81158: F, t81162: F, t81167: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F, t43888: F, t56236: F, t58073: F, t58075: F, t58090: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t81242: F, t81245: F, t1132: F, t1139: F, t43771: F, t44039: F, t44040: F, t44348: F, t52011: F, t77513: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t81336, t81338, t81341, t81343, t81350) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3076::<F>(t1196, t20890, t58647, t24473, t3531, t24764, t5206, t20400, t5207, t20692, t29322, t5023, t5501, t73252, t81322, t81326, t81328, t81330, t81333);
        let (t81352, t81379) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3077::<F>(t24324, t3379, t43881, t68255, t68257, t68262, t68277, t81156, t81158, t81162, t81167, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
        let t81397 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3078::<F>(t43888, t56236, t58073, t58075, t58090, t68332, t68334, t68336, t68389, t68399, t68454, t68456, t81224, t81228, t81230, t81232, t81234, t81236, t81242, t81245);
        let (t81399, t81401, t81403) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3079::<F>(t81379, t81397, t1132, t1139, t43771, t44039, t44040, t68255, t68257, t81156, t81158, t81162, t81167);
        let (t81416, t81418) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3080::<F>(t44348, t52011, t77513, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
    (t81336, t81338, t81341, t81343, t81350, t81352, t81399, t81401, t81403, t81416, t81418)
}
