//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta191 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1143;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1144;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1145;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1146;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1147;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1148;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1149;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1150;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1151;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1152;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1153;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta191(t467: f64, t5390: f64, t1264: f64, t5056: f64, t247: f64, t3629: f64, t5351: f64, t3626: f64, t3627: f64, t471: f64, t1715: f64, t1227: f64, t1261: f64, t1266: f64, t1808: f64, t3625: f64, t3647: f64, t3686: f64, t3705: f64, t5373: f64, t5379: f64, t5381: f64, t5384: f64, t5386: f64, t1247: f64, t1252: f64, t1797: f64, t3708: f64, t3711: f64, t484: f64, t5254: f64, t5256: f64, t5258: f64, t5262: f64, t5266: f64, t5270: f64, t5274: f64, t5279: f64, t5287: f64, t5338: f64, t5372: f64, t225: f64, t494: f64, t1811: f64, t460: f64, t1214: f64, t1828: f64, t1277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5391 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1143(t467, t5390);
        let t5397 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1144(t1264, t5056, t247);
        let t5401 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1145(t3629, t5351);
        let t5402 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1146(t3626, t5401);
        let t5405 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1147(t3627, t471);
        let t5406 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1148(t1715, t5405);
        let t5407 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1149(t3626, t5406);
        let t5410 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1150(t1227, t1261, t1266, t1808, t3625, t3647, t3686, t3705, t5373, t5379, t5381, t5384, t5386, t5391, t5397, t5402, t5407);
        let t5412 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1151(t1247, t1252, t1261, t1797, t3708, t3711, t484, t5254, t5256, t5258, t5262, t5266, t5270, t5274, t5279, t5287, t5338, t5372, t5410);
        let (t5414, t5417) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1152(t225, t494, t5412, t1811, t460);
        let (t5422, t5423) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1153(t1214, t1828, t1277);
    (t5391, t5397, t5401, t5402, t5405, t5406, t5407, t5412, t5414, t5417, t5422, t5423)
}
