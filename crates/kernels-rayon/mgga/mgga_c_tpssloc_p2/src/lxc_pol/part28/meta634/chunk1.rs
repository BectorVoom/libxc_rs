//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2008/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2008(t1375: f64, t16436: f64, t2091: f64, t3887: f64, t80689: f64, t90521: f64, t90527: f64, t90530: f64, t90539: f64, t93350: f64, t93353: f64, t93359: f64, t93361: f64, t93362: f64) -> f64 {
    let t93363 = -0.25587863262083522346e0_f64 * t90521 - t93350 - 0.3289868133696452873e-1_f64 * t90527 - 0.6579736267392905746e-1_f64 * t90530 + t93353 + 2.0_f64 * t1375 * t3887 * t2091 * t16436 + 0.3289868133696452873e-1_f64 * t90539 + t93359 + 0.38381794893125283518e-1_f64 * t80689 + t93361 - t93362;
    t93363
}
