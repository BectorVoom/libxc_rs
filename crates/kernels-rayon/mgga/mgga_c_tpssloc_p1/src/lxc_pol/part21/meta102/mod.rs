//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta102 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk708;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk709;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk710;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk711;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk712;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta102(t2531: f64, t763: f64, t2504: f64, t739: f64, t746: f64, t761: f64, t40: f64, t52: f64, t718: f64, t751: f64, t2244: f64, t2250: f64, t75: f64, t767: f64, t771: f64, t78: f64, zeta_threshold: f64, t15: f64, t60: f64, t59: f64, t207: f64, t215: f64, t782: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2532, t2533, t2535) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk708(t2531, t763, t2504, t739, t746);
        let t2537 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk709(t2535, t761);
        let (t2538, t2539, t2553) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk710(t40, t52, t718, t751, t2244, t2250, t75, t767, t771, t78, zeta_threshold);
        let t2558 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk711(t15, t60);
        let t2559 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk712(t2558, t59);
        let (t2562, t2563) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk713(t207, t215, t2559, t782, t786);
    (t2532, t2533, t2535, t2537, t2538, t2539, t2553, t2558, t2559, t2562, t2563)
}
