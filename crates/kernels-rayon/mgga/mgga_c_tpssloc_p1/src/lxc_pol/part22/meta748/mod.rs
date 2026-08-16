//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta748 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2500;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2501;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta748(t25: f64, t265: f64, t394: f64, t68418: f64, t68765: f64, t68897: f64, t68931: f64, t68999: f64, t69031: f64, t69462: f64, t69464: f64, t71055: f64, t1074: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t16557: f64, t16558: f64, t17133: f64, t18176: f64, t20216: f64, t20217: f64, t21076: f64, t21703: f64, t396: f64, t3966: f64, t40: f64, t4324: f64, t4705: f64, t5397: f64, t5398: f64, t5955: f64, t606: f64, t607: f64, t67059: f64, t67060: f64, t68427: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t18255: f64, t51667: f64, t18259: f64, t50819: f64, t22408: f64, t3640: f64, t1164: f64, t15218: f64, t18279: f64, t18910: f64, t18274: f64, t51651: f64, t18915: f64, t4875: f64, t1155: f64, t1694: f64, t18615: f64, t51848: f64, t47774: f64, t51002: f64, t68513: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t71077 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2500(t25, t265, t394, t68418, t68765, t68897, t68931, t68999, t69031, t69462, t69464, t71055, t1074, t1408, t1409, t1534, t1642, t16557, t16558, t17133, t18176, t20216, t20217, t21076, t21703, t396, t3966, t40, t4324, t4705, t5397, t5398, t5955, t606, t607, t67059, t67060, t68427, t873, dens_threshold, rho0, zeta_threshold);
        let (t71090, t71095, t71097, t71101, t71106, t71109, t71112) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2501(t67059, t18255, t51667, t18259, t50819, t22408, t3640, t1164, t15218, t18279, t18910, t18274, t51651);
        let (t71114, t71115, t71118, t71124) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2502(t18915, t4875, t1155, t1694, t18615, t51848, t47774, t51002, t68513);
    (t71077, t71090, t71095, t71097, t71101, t71106, t71109, t71112, t71114, t71115, t71118, t71124)
}
