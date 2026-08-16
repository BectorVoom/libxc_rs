//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta82 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk475;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk476;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk477;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk478;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk479;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk480;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta82(t738: f64, t172: f64, t180: f64, t2369: f64, t118: f64, t168: f64, t181: f64, t2393: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2454: f64, t2460: f64, t2462: f64, t2472: f64, t2477: f64, t2480: f64, t2486: f64, t2490: f64, t2494: f64, t2495: f64, t2505: f64, t268: f64, t725: f64, t732: f64, t740: f64, t747: f64, t157: f64, t153: f64, t145: f64, t2447: f64, t185: f64, t193: f64, t2373: f64, t2377: f64, t2378: f64, t2379: f64, t2429: f64, t2432: f64, t2450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2508 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk475(t738);
        let t2509 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk476(t2508);
        let t2510 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk477(t172, t2509);
        let (t2511, t2512) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk478(t180);
        let (t2513, t2516) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk479(t2369, t2512, t118, t168, t181, t2393, t2408, t2417, t2423, t2426, t2454, t2460, t2462, t2472, t2477, t2480, t2486, t2490, t2494, t2495, t2505, t2510, t268, t725, t732, t740, t747);
        let t2517 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk480(t157, t2516);
        let (t2518, t2519, t2520, t2521) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk481(t153, t2517, t145, t2447, t185, t193, t2373, t2377, t2378, t2379, t2408, t2417, t2423, t2426, t2429, t2432, t2450);
    (t2508, t2509, t2510, t2511, t2512, t2513, t2516, t2517, t2518, t2519, t2520, t2521)
}
