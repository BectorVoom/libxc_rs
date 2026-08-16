//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2012/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2012(t90707: f64, t90749: f64, t90759: f64, t90781: f64, t90789: f64, t90791: f64, t90794: f64, t90797: f64, t12240: f64, t16033: f64, t27074: f64, t27078: f64, t5334: f64, t90747: f64, t90752: f64, t90757: f64, t90763: f64, t90770: f64, t90774: f64, t90778: f64, t90785: f64) -> (f64, f64) {
    let t93467 = 0.76763589786250567036e-1_f64 * t90707;
    let t93473 = 0.15352717957250113407e0_f64 * t90749;
    let t93476 = 0.76763589786250567036e-1_f64 * t90759;
    let t93483 = 0.16449340668482264365e-1_f64 * t90781;
    let t93488 = 0.9869604401089358619e-1_f64 * t90789;
    let t93489 = 0.15352717957250113407e0_f64 * t90791;
    let t93490 = 0.3289868133696452873e-1_f64 * t90794;
    let t93491 = 0.3289868133696452873e-1_f64 * t90797;
    let t93492 = 0.3289868133696452873e-1_f64 * t90747 - t93473 - 0.16449340668482264365e-1_f64 * t90752 + 0.19739208802178717238e0_f64 * t90757 + t93476 - 0.9869604401089358619e-1_f64 * t90763 - 0.3289868133696452873e-1_f64 * t90770 - 2.0_f64 * t16033 * t27078 + 0.6579736267392905746e-1_f64 * t90774 + 0.3289868133696452873e-1_f64 * t90778 + t93483 + 2.0_f64 * t5334 * t27074 * t12240 - 0.16449340668482264365e-1_f64 * t90785 - t93488 + t93489 + t93490 + t93491;
    (t93467, t93492)
}
