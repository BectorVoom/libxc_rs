//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta83 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk482;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk483;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk484;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk485;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk486;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta83(t193: f64, t201: f64, t868: f64, t870: f64, t2369: f64, t2509: f64, t2512: f64, t761: f64, t172: f64, t753: f64, t763: f64, t2504: f64, t739: f64, t746: f64, t40: f64, t52: f64, t718: f64, t751: f64, t2244: f64, t2250: f64, t75: f64, t767: f64, t771: f64, t78: f64, zeta_threshold: f64, t15: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2522 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk482(t193, t201);
        let (t2523, t2527, t2528) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk483(t868, t870, t2369, t2509, t2512);
        let (t2530, t2531) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk484(t2528, t761, t172, t753);
        let (t2533, t2535) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk485(t2531, t763, t2504, t739, t746);
        let (t2537, t2539, t2553) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk486(t40, t52, t2535, t761, t718, t751, t2244, t2250, t75, t767, t771, t78, zeta_threshold);
        let t2558 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk487(t15, t60);
    (t2522, t2523, t2527, t2528, t2530, t2531, t2533, t2535, t2537, t2539, t2553, t2558)
}
