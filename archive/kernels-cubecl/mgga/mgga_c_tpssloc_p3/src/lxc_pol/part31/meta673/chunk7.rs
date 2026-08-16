//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2031/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2031<F: Float>(t1375: F, t16030: F, t20022: F, t20050: F, t20060: F, t2091: F, t2092: F, t26224: F, t27115: F, t27132: F, t29361: F, t3882: F, t3887: F, t5321: F, t5353: F, t56640: F, t7214: F, t7936: F, t7937: F, t90743: F, t93319: F, t93824: F, t97571: F, t97573: F, t97577: F, t97583: F, t97588: F, t97599: F, t97604: F, t97611: F, t97616: F) -> F {
    let t102861 = -F::cast_from(2.0_f64) * t5321 * t27115 + t93824 - F::cast_from(0.16449340668482264365e-1_f64) * t97571 + F::cast_from(0.76763589786250567037e-1_f64) * t97573 + F::cast_from(0.6579736267392905746e-1_f64) * t97577 - F::cast_from(2.0_f64) * t16030 * t7937 - t20060 * t7214 + F::cast_from(4.0_f64) * t1375 * t3887 * t7936 * t5353 + F::cast_from(24.0_f64) * t26224 * t93319 * t20050 - F::cast_from(0.13159472534785811492e0_f64) * t97583 - t56640 * t2092 - F::cast_from(0.39478417604357434476e0_f64) * t97588 + F::cast_from(4.0_f64) * t5321 * t27132 - t3882 * t29361 - F::cast_from(0.82246703342411321825e-2_f64) * t97599 + F::cast_from(0.16449340668482264365e-1_f64) * t97604 - t90743 - F::cast_from(0.6579736267392905746e-1_f64) * t97611 + F::cast_from(2.0_f64) * t1375 * t3887 * t2091 * t20022 - F::cast_from(0.3289868133696452873e-1_f64) * t97616;
    t102861
}
