//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1341;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1342;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1343;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1344;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta287(t2476: f64, t676: f64, t2504: f64, t2512: f64, t745: f64, t747: f64, t2405: f64, t2411: f64, t2414: f64, t701: f64, t118: f64, t142: f64, t9697: f64, t181: f64, t2454: f64, t2459: f64, t2460: f64, t2462: f64, t2471: f64, t2472: f64, t2477: f64, t2479: f64, t2480: f64, t2490: f64, t2494: f64, t2495: f64, t2505: f64, t2510: f64, t2513: f64, t268: f64, t730: f64, t732: f64, t9799: f64, t9803: f64, t9810: f64, t9814: f64, t9820: f64, t9824: f64, t9798: f64, t157: f64, t153: f64, t2371: f64, t2531: f64, t2528: f64, t2517: f64, t607: f64, t707: f64, t2652: f64, t2663: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9828, t9843, t9844, t9847, t9853) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1341(t2476, t676, t2504, t2512, t745, t747, t2405, t2411, t2414, t701);
        let t9859 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1342(t118, t142, t9697);
        let t9860 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1343(t118, t181, t2454, t2459, t2460, t2462, t2471, t2472, t2477, t2479, t2480, t2490, t2494, t2495, t2505, t2510, t2513, t268, t676, t730, t732, t747, t9697, t9799, t9803, t9810, t9814, t9820, t9824, t9828, t9844, t9847, t9853, t9859);
        let (t9861, t9863, t9864, t9866, t9869, t9871) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1344(t9798, t9860, t157, t153, t2371, t2531, t2528, t2517, t607, t707, t2652, t2663);
    (t9843, t9853, t9859, t9861, t9863, t9864, t9866, t9869, t9871)
}
