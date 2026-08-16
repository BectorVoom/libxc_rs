//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1420;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1421;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1422;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1423;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1424;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta393(t16696: f64, t5332: f64, t3720: f64, t12772: f64, t5406: f64, t3625: f64, t1248: f64, t5245: f64, t1250: f64, t1802: f64, t474: f64, t3089: f64, t3717: f64, t1261: f64, t12809: f64, t12832: f64, t17362: f64, t17369: f64, t17375: f64, t17377: f64, t3613: f64, t3647: f64, t3718: f64, t3723: f64, t5348: f64, t5354: f64, t5397: f64, t1284: f64, t5219: f64, t3624: f64, t1225: f64, t13312: f64, t1012: f64, t1230: f64, t5390: f64, t12879: f64, t1715: f64, t247: f64, t16756: f64, t5341: f64, t12916: f64, t5342: f64, t5340: f64, t12702: f64, t5330: f64, t12744: f64, t1222: f64, t1266: f64, t12853: f64, t3689: f64, t3694: f64, t5335: f64, t5343: f64, t5373: f64, t127: f64, t371: f64, t5318: f64, t1235: f64, t1803: f64, t3670: f64, t3685: f64, t140: f64, t5368: f64, t5436: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17381, t17386, t17389, t17391, t17395) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1420(t16696, t5332, t3720, t12772, t5406, t3625, t1248, t5245, t1250, t1802, t474, t3089);
        let t17399 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1421(t17395, t3717, t1261, t12809, t12832, t17362, t17369, t17375, t17377, t17381, t17386, t17391, t3613, t3647, t3718, t3723, t5348, t5354, t5397);
        let (t17401, t17405, t17412, t17417) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1422(t1284, t5219, t3624, t1225, t13312, t1012, t1230, t5390, t12879, t1715, t247, t1261);
        let t17432 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1423(t16756, t5341, t3720, t12916, t5342, t5340, t12702, t5330, t12744, t1222, t1266, t12853, t17401, t17405, t17412, t17417, t3689, t3694, t3723, t5335, t5343, t5373);
        let (t17437, t17438, t17444, t17447, t17448) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1424(t127, t371, t5318, t1235, t1803, t3670, t3685, t5373, t140, t5368, t1222, t3624, t5436);
    (t17389, t17395, t17399, t17432, t17437, t17438, t17444, t17447, t17448)
}
