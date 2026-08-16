//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta106 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk579;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk580;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk581;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk582;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk583;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk584;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta106(t3014: f64, t340: f64, t343: f64, t974: f64, t2955: f64, t2958: f64, t2960: f64, t2969: f64, t2972: f64, t2975: f64, t2982: f64, t2986: f64, t2991: f64, t2996: f64, t3000: f64, t3011: f64, t346: f64, t973: f64, t980: f64, t987: f64, t381: f64, t1049: f64, t990: f64, t225: f64, t991: f64, t1008: f64, t191: f64, t349: f64, t1011: f64, t68: f64, t371: f64, t335: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3016, t3017, t3020) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk579(t3014, t340, t343, t974, t2955, t2958, t2960, t2969, t2972, t2975, t2982, t2986, t2991, t2996, t3000, t3011, t346, t973, t980, t987);
        let (t3021, t3023, t3026, t3030) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk580(t3020, t381, t1049, t990, t225, t991, t1008, t191);
        let (t3031, t3032) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk581(t3030, t349, t1011, t68);
        let t3033 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk582(t3031, t3032);
        let t3034 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk583(t371);
        let t3036 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk584(t3034, t335);
    (t3016, t3017, t3020, t3021, t3023, t3026, t3030, t3031, t3032, t3033, t3034, t3036)
}
