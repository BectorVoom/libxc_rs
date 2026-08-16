//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta81 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk474;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk475;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk476;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk477;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk478;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk479;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk480;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta81(t2528: f64, t761: f64, t2504: f64, t739: f64, t746: f64, t15: f64, t60: f64, t59: f64, t207: f64, t215: f64, t782: f64, t786: f64, t591: f64, t795: f64, t154: f64, t244: f64, t205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2530, t2535) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk474(t2528, t761, t2504, t739, t746);
        let (t2537, t2558) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk475(t2535, t761, t15, t60);
        let t2559 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk476(t2558, t59);
        let (t2562, t2563) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk477(t207, t215, t2559, t782, t786);
        let t2566 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk478(t59, t591);
        let (t2569, t2570) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk479(t207, t2566, t795, t154, t244);
        let t2571 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk480(t205, t2570);
    (t2530, t2535, t2537, t2558, t2559, t2562, t2563, t2566, t2569, t2570, t2571)
}
