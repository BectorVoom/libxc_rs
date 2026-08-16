//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta773 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2644;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2645;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta773(t28: f64, t265: f64, t504: f64, t68418: f64, t71222: f64, t71252: f64, t72059: f64, t72074: f64, t72077: f64, t72078: f64, t72099: f64, t73931: f64, t1081: f64, t1260: f64, t1409: f64, t1534: f64, t1649: f64, t16558: f64, t17133: f64, t1768: f64, t18196: f64, t19276: f64, t20217: f64, t20390: f64, t21076: f64, t22414: f64, t3966: f64, t4324: f64, t506: f64, t5099: f64, t52: f64, t5398: f64, t5966: f64, t607: f64, t6279: f64, t67060: f64, t68427: f64, t71090: f64, t873: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t53777: f64, t53779: f64, t56099: f64, t56102: f64, t56104: f64, t20396: f64, t67: f64, t758: f64, t53798: f64, t5397: f64, t606: f64, t584: f64, t25: f64, t15937: f64, t15940: f64, t16557: f64, t19547: f64, t20216: f64, t20376: f64, t2219: f64, t3664: f64, t39419: f64, t5134: f64, t514: f64, t67059: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t73953 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2644(t28, t265, t504, t68418, t71222, t71252, t72059, t72074, t72077, t72078, t72099, t73931, t1081, t1260, t1409, t1534, t1649, t16558, t17133, t1768, t18196, t19276, t20217, t20390, t21076, t22414, t3966, t4324, t506, t5099, t52, t5398, t5966, t607, t6279, t67060, t68427, t71090, t873, dens_threshold, rho1, zeta_threshold);
        let (t73958, t73959, t73960, t73961, t73962, t73968, t73969, t73975, t73978) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2645(t53777, t53779, t56099, t56102, t56104, t20396, t67, t758, t53798, t5397, t606, t584);
        let t73989 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2646(t25, t15937, t15940, t16557, t19547, t20216, t20376, t2219, t3664, t39419, t5134, t514, t606, t67059, t73975, t73978, zeta_threshold);
    (t73953, t73958, t73959, t73960, t73961, t73962, t73968, t73969, t73975, t73978, t73989)
}
