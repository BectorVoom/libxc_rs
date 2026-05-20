//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2132;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2133;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2134;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta563<F: Float>(t10355: F, t22688: F, t4201: F, t5825: F, t22671: F, t48: F, t477: F, t53: F, t10368: F, t4210: F, t60: F, t10379: F, t1480: F, t1483: F, t44: F, t56: F, t5843: F, t5848: F, t5851: F, t61: F, sigma2: F, t38: F, t10389: F, t10398: F, t4227: F, t4232: F, t633: F, t637: F, t77: F, t1471: F, t1487: F, t1494: F, t21686: F, t22662: F, t22665: F, t22673: F, t22676: F, t22681: F, t5820: F, t5827: F, t5830: F, t5855: F, t5869: F, t71: F, t85: F, t5: F, t10309: F, t13272: F, t1497: F, t21663: F, t2247: F, t22648: F, t22656: F, t22659: F, t4173: F, t5816: F, t5872: F, t603: F, t91: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22689, t22692, t22695, t22700, t22718) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2132::<F>(t10355, t22688, t4201, t5825, t22671, t48, t477, t53, t10368, t4210, t60, t10379, t1480, t1483, t44, t56, t5843, t5848, t5851, t61, sigma2);
        let (t22719, t22739) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2133::<F>(t22718, t38, t10389, t10398, t22671, t22688, t4227, t4232, t5825, t633, t637, t77);
        let t22742 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2134::<F>(t1471, t1487, t1494, t21686, t22662, t22665, t22673, t22676, t22681, t22719, t22739, t5820, t5827, t5830, t5855, t5869, t71, t85);
        let t22746 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2135::<F>(t5, t10309, t13272, t1497, t21663, t2247, t22648, t22656, t22659, t22742, t4173, t5816, t5872, t603, t91);
    (t22689, t22692, t22695, t22700, t22718, t22719, t22739, t22742, t22746)
}
