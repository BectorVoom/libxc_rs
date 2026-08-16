//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1990/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1990(t2054: f64, t24297: f64, t26690: f64, t2713: f64, t4301: f64, t46508: f64, t82143: f64, t82145: f64, t82147: f64, t82150: f64, t855: f64, t858: f64, t87033: f64, t87039: f64, t92486: f64, t92506: f64, t92528: f64, t92558: f64, t92732: f64, t92759: f64, t92782: f64, t92803: f64, t92826: f64) -> f64 {
    let t92839 = -2.0_f64 * t24297 * t4301 + t92486 + 0.38381794893125283518e-1_f64 * t82143 - 0.3289868133696452873e-1_f64 * t87033 - t855 * t858 * (t92506 + t92528 + t92558 + t92732 + t92759 + t92782 + t92803 + t92826) - t46508 * t2054 - 0.13159472534785811492e0_f64 * t87039 + 0.76763589786250567036e-1_f64 * t82145 - 0.10417915756705434098e0_f64 * t82147 + 0.76763589786250567036e-1_f64 * t82150 + 4.0_f64 * t2713 * t26690;
    t92839
}
