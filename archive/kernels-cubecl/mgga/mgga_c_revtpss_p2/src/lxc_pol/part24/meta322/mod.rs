//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta322 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1114;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1115;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1116;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1117;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta322<F: Float>(t22718: F, t38: F, t10389: F, t10398: F, t22671: F, t22688: F, t4227: F, t4232: F, t5825: F, t633: F, t637: F, t77: F, t1471: F, t1487: F, t1494: F, t21686: F, t22662: F, t22665: F, t22673: F, t22676: F, t22681: F, t5820: F, t5827: F, t5830: F, t5855: F, t5869: F, t71: F, t85: F, t5: F, t10309: F, t13272: F, t1497: F, t21663: F, t2247: F, t22648: F, t22656: F, t22659: F, t4173: F, t5816: F, t5872: F, t603: F, t91: F, t117: F, t1312: F, t1518: F, t18245: F, t22633: F, t22639: F, t4248: F, t5920: F, t7889: F, t13584: F, t22186: F, t22188: F, t22191: F, t22196: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22719, t22739) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1114::<F>(t22718, t38, t10389, t10398, t22671, t22688, t4227, t4232, t5825, t633, t637, t77);
        let t22742 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1115::<F>(t1471, t1487, t1494, t21686, t22662, t22665, t22673, t22676, t22681, t22719, t22739, t5820, t5827, t5830, t5855, t5869, t71, t85);
        let t22746 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1116::<F>(t5, t10309, t13272, t1497, t21663, t2247, t22648, t22656, t22659, t22742, t4173, t5816, t5872, t603, t91);
        let (t22747, t22758, t22762, t22763) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1117::<F>(t117, t22746, t1312, t1518, t18245, t22633, t22639, t4248, t5920, t7889, t13584, t22186);
        let (t22764, t22765, t22766, t22767) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1118::<F>(t22188, t22191, t22196, t22762, t22763, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t22719, t22739, t22742, t22746, t22747, t22758, t22762, t22763, t22764, t22765, t22766, t22767)
}
