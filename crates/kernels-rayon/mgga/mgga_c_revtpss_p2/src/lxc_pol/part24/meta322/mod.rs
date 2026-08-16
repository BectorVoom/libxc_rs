//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1114;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1115;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1116;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1117;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta322(t22718: f64, t38: f64, t10389: f64, t10398: f64, t22671: f64, t22688: f64, t4227: f64, t4232: f64, t5825: f64, t633: f64, t637: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t21686: f64, t22662: f64, t22665: f64, t22673: f64, t22676: f64, t22681: f64, t5820: f64, t5827: f64, t5830: f64, t5855: f64, t5869: f64, t71: f64, t85: f64, t5: f64, t10309: f64, t13272: f64, t1497: f64, t21663: f64, t2247: f64, t22648: f64, t22656: f64, t22659: f64, t4173: f64, t5816: f64, t5872: f64, t603: f64, t91: f64, t117: f64, t1312: f64, t1518: f64, t18245: f64, t22633: f64, t22639: f64, t4248: f64, t5920: f64, t7889: f64, t13584: f64, t22186: f64, t22188: f64, t22191: f64, t22196: f64, t9278: f64, t9308: f64, t9316: f64, t9320: f64, t9325: f64, t9329: f64, t9333: f64, t9374: f64, t9389: f64, t9391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22719, t22739) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1114(t22718, t38, t10389, t10398, t22671, t22688, t4227, t4232, t5825, t633, t637, t77);
        let t22742 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1115(t1471, t1487, t1494, t21686, t22662, t22665, t22673, t22676, t22681, t22719, t22739, t5820, t5827, t5830, t5855, t5869, t71, t85);
        let t22746 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1116(t5, t10309, t13272, t1497, t21663, t2247, t22648, t22656, t22659, t22742, t4173, t5816, t5872, t603, t91);
        let (t22747, t22758, t22762, t22763) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1117(t117, t22746, t1312, t1518, t18245, t22633, t22639, t4248, t5920, t7889, t13584, t22186);
        let (t22764, t22765, t22766, t22767) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1118(t22188, t22191, t22196, t22762, t22763, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t22719, t22739, t22742, t22746, t22747, t22758, t22762, t22763, t22764, t22765, t22766, t22767)
}
