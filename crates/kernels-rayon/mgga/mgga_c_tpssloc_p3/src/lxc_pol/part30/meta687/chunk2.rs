//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2179/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2179(t1375: f64, t1386: f64, t16460: f64, t19647: f64, t20044: f64, t20050: f64, t20060: f64, t22670: f64, t26224: f64, t26225: f64, t26371: f64, t26472: f64, t26482: f64, t3887: f64, t5215: f64, t5321: f64, t5353: f64, t6461: f64, t6963: f64, t6993: f64, t7749: f64, t7750: f64, t81311: f64, t90696: f64, t90724: f64, t97558: f64, t97571: f64, t97573: f64, t97577: f64, t97583: f64, t97588: f64, t97599: f64, t97604: f64) -> f64 {
    let t97607 = 4.0_f64 * t5215 * t26371 - t97558 * t1386 - 12.0_f64 * t26224 * t26225 * t19647 - 2.0_f64 * t5321 * t26472 - t20060 * t6993 + 2.0_f64 * t20044 * t6963 - 2.0_f64 * t16460 * t7750 + t90724 - 0.82246703342411321825e-2_f64 * t97571 + 0.38381794893125283518e-1_f64 * t97573 + 0.3289868133696452873e-1_f64 * t97577 + 4.0_f64 * t5215 * t26482 - 0.6579736267392905746e-1_f64 * t97583 - 0.19739208802178717238e0_f64 * t97588 - t22670 * t6461 + 4.0_f64 * t1375 * t3887 * t7749 * t5353 + 24.0_f64 * t26224 * t90696 * t20050 - 0.41123351671205660912e-2_f64 * t97599 + 0.82246703342411321825e-2_f64 * t97604 - 0.82246703342411321824e-2_f64 * t81311;
    t97607
}
