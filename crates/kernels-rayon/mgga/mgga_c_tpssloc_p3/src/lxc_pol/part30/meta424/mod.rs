//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta424 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1639;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1640;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1641;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1642;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1643;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1644;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta424(t19482: f64, t666: f64, t5468: f64, t9384: f64, t659: f64, t1444: f64, t2: f64, t584: f64, t2341: f64, t5396: f64, t9212: f64, t95: f64, t5480: f64, t9398: f64, t662: f64, t1449: f64, t2349: f64, t5484: f64, t103: f64, t100: f64, t12774: f64, t12795: f64, t1447: f64, t4060: f64, t4064: f64, t5469: f64, t5472: f64, t5475: f64, t657: f64, t663: f64, t92: f64, t656: f64, t12747: f64, t12750: f64, t12752: f64, t19471: f64, t19474: f64, t19477: f64, t19480: f64, t64: f64, t9358: f64, t9359: f64, t109: f64, t1268: f64, t12725: f64, t1458: f64, t19450: f64, t19451: f64, t19456: f64, t19461: f64, t2314: f64, t4028: f64, t4072: f64, t5113: f64, t5493: f64, t671: f64, t7676: f64, t25: f64, t6320: f64, t67: f64, t758: f64, t12061: f64, t6305: f64, t3664: f64, t5397: f64, t16557: f64, t2219: f64, t5134: f64, t514: f64, t606: f64, zeta_threshold: f64, t28: f64, t12072: f64, t6312: f64, t3672: f64, t5966: f64, t1081: f64, t18196: f64, t5142: f64, t517: f64, t157: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t19483, t19489, t19493, t19499, t19503, t19504) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1639(t19482, t666, t5468, t9384, t659, t1444, t2, t584, t2341, t5396, t9212, t95);
        let t19529 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1640(t5480, t9398, t662, t1449, t2, t584, t2349, t5484, t19503, t103, t100, t12774, t12795, t1447, t19489, t19493, t19499, t19504, t4060, t4064, t5469, t5472, t5475, t657, t663, t92);
        let t19533 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1641(t19529, t656, t12747, t12750, t12752, t19471, t19474, t19477, t19480, t19483, t64, t9358, t9359);
        let t19534 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1642(t109, t19533);
        let t19537 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1643(t1268, t12725, t1458, t19450, t19451, t19456, t19461, t19534, t2314, t4028, t4072, t5113, t5493, t671, t7676);
        let (t19543, t19558) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1644(t25, t6320, t67, t758, t12061, t6305, t3664, t5397, t16557, t2219, t5134, t514, t606, zeta_threshold);
        let (t19572, t19573) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1645(t28, t12072, t6312, t3672, t5966, t1081, t18196, t2219, t5142, t517, t157, t19558, t184, zeta_threshold);
    (t19529, t19534, t19537, t19543, t19572, t19573)
}
