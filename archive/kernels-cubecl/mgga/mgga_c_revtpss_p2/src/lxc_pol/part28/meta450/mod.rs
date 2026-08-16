//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta450 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1699;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1700;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1701;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1702;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta450<F: Float>(t16696: F, t5332: F, t3720: F, t12772: F, t5406: F, t3625: F, t1248: F, t5245: F, t1250: F, t1802: F, t474: F, t3089: F, t3717: F, t1261: F, t12809: F, t12832: F, t17362: F, t17369: F, t17375: F, t17377: F, t3613: F, t3647: F, t3718: F, t3723: F, t5348: F, t5354: F, t5397: F, t1284: F, t5219: F, t3624: F, t1225: F, t13312: F, t1012: F, t1230: F, t5390: F, t12879: F, t1715: F, t247: F, t16756: F, t5341: F, t12916: F, t5342: F, t5340: F, t12702: F, t5330: F, t12744: F, t1222: F, t1266: F, t12853: F, t3689: F, t3694: F, t5335: F, t5343: F, t5373: F, t127: F, t371: F, t5318: F, t1235: F, t1803: F, t3670: F, t3685: F, t140: F, t5368: F, t5436: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17381, t17386, t17389, t17391, t17395) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1699::<F>(t16696, t5332, t3720, t12772, t5406, t3625, t1248, t5245, t1250, t1802, t474, t3089);
        let t17399 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1700::<F>(t17395, t3717, t1261, t12809, t12832, t17362, t17369, t17375, t17377, t17381, t17386, t17391, t3613, t3647, t3718, t3723, t5348, t5354, t5397);
        let (t17401, t17405, t17412, t17417) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1701::<F>(t1284, t5219, t3624, t1225, t13312, t1012, t1230, t5390, t12879, t1715, t247, t1261);
        let t17432 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1702::<F>(t16756, t5341, t3720, t12916, t5342, t5340, t12702, t5330, t12744, t1222, t1266, t12853, t17401, t17405, t17412, t17417, t3689, t3694, t3723, t5335, t5343, t5373);
        let (t17437, t17438, t17444, t17447, t17448) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1703::<F>(t127, t371, t5318, t1235, t1803, t3670, t3685, t5373, t140, t5368, t1222, t3624, t5436);
    (t17389, t17395, t17399, t17432, t17437, t17438, t17444, t17447, t17448)
}
