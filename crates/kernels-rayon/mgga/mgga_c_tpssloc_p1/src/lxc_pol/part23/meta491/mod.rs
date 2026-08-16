//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1502;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1503;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1504;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1505;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1506;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1507;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta491(t80019: f64, t80047: f64, t6414: f64, t550: f64, t3792: f64, t39249: f64, t39256: f64, t39261: f64, t39266: f64, t39304: f64, t39309: f64, t79834: f64, t79835: f64, t79836: f64, t79837: f64, t79853: f64, t79854: f64, t39312: f64, t39316: f64, t39320: f64, t39324: f64, t39327: f64, t39338: f64, t39346: f64, t39349: f64, t39356: f64, t79856: f64, t79857: f64, t79858: f64, t79890: f64, t39360: f64, t39364: f64, t39373: f64, t39384: f64, t39393: f64, t39397: f64, t39400: f64, t39408: f64, t79896: f64, t79897: f64, t79898: f64, t79899: f64, t39411: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t79904: f64, t79905: f64, t79906: f64, t79907: f64, t79908: f64, t79909: f64, t79910: f64, t39490: f64, t39496: f64, t39499: f64, t39502: f64, t39505: f64, t39508: f64, t39518: f64, t39521: f64, t39529: f64, t39539: f64, t39549: f64, t79914: f64, t39563: f64, t39570: f64, t39582: f64, t39585: f64, t39590: f64, t39593: f64, t39595: f64, t79925: f64, t79927: f64, t79928: f64, t79929: f64, t79930: f64, t79934: f64, t39597: f64, t39604: f64, t39606: f64, t39608: f64, t39615: f64, t39635: f64, t79935: f64, t79942: f64, t79946: f64, t79952: f64, t79953: f64, t79954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t80048, t80076, t80085, t80101) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1502(t80019, t80047, t6414, t550, t3792, t39249, t39256, t39261, t39266, t39304, t39309, t79834, t79835, t79836, t79837, t79853, t79854);
        let t80102 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1503(t39312, t39316, t39320, t39324, t39327, t39338, t39346, t39349, t39356, t79856, t79857, t79858, t79890);
        let t80104 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1504(t39360, t39364, t39373, t39384, t39393, t39397, t39400, t39408, t79896, t79897, t79898, t79899);
        let t80105 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1505(t39411, t39463, t39468, t39472, t39476, t39483, t79904, t79905, t79906, t79907, t79908, t79909, t79910);
        let t80108 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1506(t39490, t39496, t39499, t39502, t39505, t39508, t39518, t39521, t39529, t39539, t39549, t79914);
        let t80109 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1507(t39563, t39570, t39582, t39585, t39590, t39593, t39595, t79925, t79927, t79928, t79929, t79930, t79934);
        let t80111 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1508(t39597, t39604, t39606, t39608, t39615, t39635, t79935, t79942, t79946, t79952, t79953, t79954);
    (t80048, t80076, t80085, t80101, t80102, t80104, t80105, t80108, t80109, t80111)
}
