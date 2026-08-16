//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 547/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk547(t2936: f64, t300: f64, t2898: f64, t938: f64, t961: f64, t2904: f64, t2906: f64, t951: f64, t959: f64, t2924: f64, t942: f64, t2929: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2937 = t300 * t2936;
    let t2939 = 0.19751673498613801407e-1_f64 * t300 * t2898;
    let t2940 = t300 * t938;
    let t2942 = 0.11696447245269292414e1_f64 * t2940 * t961;
    let t2944 = t2904 * t2906 * t951;
    let t2946 = 0.11696447245269292414e1_f64 * t959 * t2944;
    let t2948 = t942 * t2924 * t951;
    let t2950 = 0.5848223622634646207e0_f64 * t959 * t2948;
    let t2951 = t2929 * t2906;
    (t2937, t2939, t2940, t2942, t2944, t2946, t2948, t2950, t2951)
}
