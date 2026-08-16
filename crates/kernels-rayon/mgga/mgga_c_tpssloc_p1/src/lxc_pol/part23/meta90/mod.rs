//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk512;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk513;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk514;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk515;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta90(t343: f64, t883: f64, t2775: f64, t344: f64, t2822: f64, t1008: f64, t191: f64, t349: f64, t1011: f64, t68: f64, t371: f64, t335: f64, t368: f64, t1015: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2989, t2994, t3003, t3030) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk512(t343, t883, t2775, t344, t2822, t1008, t191);
        let (t3031, t3032) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk513(t3030, t349, t1011, t68);
        let (t3033, t3034) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk514(t3031, t3032, t371);
        let t3036 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk515(t3034, t335);
        let (t3037, t3038, t3039) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk516(t3036, t368, t1015, t3033);
    (t2989, t2994, t3003, t3030, t3031, t3032, t3033, t3034, t3036, t3037, t3038, t3039)
}
