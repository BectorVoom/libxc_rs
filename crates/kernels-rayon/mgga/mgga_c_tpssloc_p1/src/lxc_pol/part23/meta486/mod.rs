//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1490;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1491;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta486(t25: f64, t54312: f64, t54314: f64, t54316: f64, t6305: f64, t5397: f64, t19547: f64, t20216: f64, t3664: f64, t39419: f64, t5134: f64, t514: f64, t75911: f64, zeta_threshold: f64, t28: f64, t6312: f64, t5966: f64, t19559: f64, t20390: f64, t3672: f64, t39436: f64, t5142: f64, t517: f64, t77953: f64, t157: f64, t182: f64, t39266: f64, t39304: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t39324: f64, t39327: f64, t39338: f64, t39346: f64, t39349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79856, t79857, t79858, t79859, t79864, t79872) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1490(t25, t54312, t54314, t54316, t6305, t5397, t19547, t20216, t3664, t39419, t5134, t514, t75911, zeta_threshold);
        let (t79873, t79878, t79888, t79890) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1491(t28, t6312, t5966, t19559, t20390, t3672, t39436, t5142, t517, t77953, t157, t79872, t182, zeta_threshold);
        let t79891 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1492(t39266, t39304, t39309, t39312, t39316, t39320, t39324, t39327, t39338, t39346, t39349, t79856, t79857, t79858, t79890);
    (t79856, t79857, t79858, t79859, t79864, t79873, t79878, t79888, t79890, t79891)
}
