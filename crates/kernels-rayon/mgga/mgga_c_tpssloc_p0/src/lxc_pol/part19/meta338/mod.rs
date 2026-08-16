//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1204;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1205;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1206;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta338(t40: f64, t52: f64, t2244: f64, t2250: f64, t2291: f64, t39097: f64, t39103: f64, t39110: f64, t634: f64, t75: f64, t767: f64, t9258: f64, t9499: f64, t2298: f64, t638: f64, t771: f64, t78: f64, t9508: f64, zeta_threshold: f64, t10047: f64, t225: f64, t2742: f64, t9587: f64, t9585: f64, t10046: f64, t10049: f64, t10104: f64, t10110: f64, t10112: f64, t10116: f64, t259: f64, t2591: f64, t2710: f64, t2713: f64, t2718: f64, t2719: f64, t2720: f64, t2743: f64, t798: f64, t855: f64, t866: f64, t9593: f64, t10108: f64, t257: f64, t68: f64, t2627: f64, t10016: f64, t252: f64, t9957: f64, t852: f64, t9971: f64, t2631: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40833, t40846) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1204(t40, t52, t2244, t2250, t2291, t39097, t39103, t39110, t634, t75, t767, t9258, t9499, t2298, t638, t771, t78, t9508, zeta_threshold);
        let t40848 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1205(t40833, t40846);
        let t40887 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1206(t10047, t225, t2742, t9587, t9585, t10046, t10049, t10104, t10110, t10112, t10116, t259, t2591, t2710, t2713, t2718, t2719, t2720, t2743, t798, t855, t866, t9593);
        let (t40890, t40891, t40895, t40904, t40909, t40917, t40925) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1207(t10108, t257, t68, t2719, t2627, t2710, t10016, t252, t9957, t852, t9971, t2631);
    (t40848, t40887, t40890, t40891, t40895, t40904, t40909, t40917, t40925)
}
