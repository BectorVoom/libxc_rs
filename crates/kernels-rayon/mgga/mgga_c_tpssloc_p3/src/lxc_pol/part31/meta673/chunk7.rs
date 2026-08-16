//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2031/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2031(t1375: f64, t16030: f64, t20022: f64, t20050: f64, t20060: f64, t2091: f64, t2092: f64, t26224: f64, t27115: f64, t27132: f64, t29361: f64, t3882: f64, t3887: f64, t5321: f64, t5353: f64, t56640: f64, t7214: f64, t7936: f64, t7937: f64, t90743: f64, t93319: f64, t93824: f64, t97571: f64, t97573: f64, t97577: f64, t97583: f64, t97588: f64, t97599: f64, t97604: f64, t97611: f64, t97616: f64) -> f64 {
    let t102861 = -2.0_f64 * t5321 * t27115 + t93824 - 0.16449340668482264365e-1_f64 * t97571 + 0.76763589786250567037e-1_f64 * t97573 + 0.6579736267392905746e-1_f64 * t97577 - 2.0_f64 * t16030 * t7937 - t20060 * t7214 + 4.0_f64 * t1375 * t3887 * t7936 * t5353 + 24.0_f64 * t26224 * t93319 * t20050 - 0.13159472534785811492e0_f64 * t97583 - t56640 * t2092 - 0.39478417604357434476e0_f64 * t97588 + 4.0_f64 * t5321 * t27132 - t3882 * t29361 - 0.82246703342411321825e-2_f64 * t97599 + 0.16449340668482264365e-1_f64 * t97604 - t90743 - 0.6579736267392905746e-1_f64 * t97611 + 2.0_f64 * t1375 * t3887 * t2091 * t20022 - 0.3289868133696452873e-1_f64 * t97616;
    t102861
}
