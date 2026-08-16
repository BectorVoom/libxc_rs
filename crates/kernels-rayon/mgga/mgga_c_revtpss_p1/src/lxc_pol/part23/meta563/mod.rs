//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2132;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2133;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2134;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta563(t10355: f64, t22688: f64, t4201: f64, t5825: f64, t22671: f64, t48: f64, t477: f64, t53: f64, t10368: f64, t4210: f64, t60: f64, t10379: f64, t1480: f64, t1483: f64, t44: f64, t56: f64, t5843: f64, t5848: f64, t5851: f64, t61: f64, sigma2: f64, t38: f64, t10389: f64, t10398: f64, t4227: f64, t4232: f64, t633: f64, t637: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t21686: f64, t22662: f64, t22665: f64, t22673: f64, t22676: f64, t22681: f64, t5820: f64, t5827: f64, t5830: f64, t5855: f64, t5869: f64, t71: f64, t85: f64, t5: f64, t10309: f64, t13272: f64, t1497: f64, t21663: f64, t2247: f64, t22648: f64, t22656: f64, t22659: f64, t4173: f64, t5816: f64, t5872: f64, t603: f64, t91: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22689, t22692, t22695, t22700, t22718) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2132(t10355, t22688, t4201, t5825, t22671, t48, t477, t53, t10368, t4210, t60, t10379, t1480, t1483, t44, t56, t5843, t5848, t5851, t61, sigma2);
        let (t22719, t22739) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2133(t22718, t38, t10389, t10398, t22671, t22688, t4227, t4232, t5825, t633, t637, t77);
        let t22742 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2134(t1471, t1487, t1494, t21686, t22662, t22665, t22673, t22676, t22681, t22719, t22739, t5820, t5827, t5830, t5855, t5869, t71, t85);
        let t22746 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2135(t5, t10309, t13272, t1497, t21663, t2247, t22648, t22656, t22659, t22742, t4173, t5816, t5872, t603, t91);
    (t22689, t22692, t22695, t22700, t22718, t22719, t22739, t22742, t22746)
}
