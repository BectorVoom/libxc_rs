//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 723/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk723(t2940: f64, t961: f64, t2904: f64, t2906: f64, t951: f64, t959: f64, t2924: f64, t942: f64, t2929: f64, t2932: f64, t2262: f64, t338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2942 = 0.11696447245269292414e1_f64 * t2940 * t961;
    let t2944 = t2904 * t2906 * t951;
    let t2946 = 0.11696447245269292414e1_f64 * t959 * t2944;
    let t2948 = t942 * t2924 * t951;
    let t2950 = 0.5848223622634646207e0_f64 * t959 * t2948;
    let t2951 = t2929 * t2906;
    let t2952 = t2951 * t2932;
    let t2954 = 0.17315859105681463759e2_f64 * t959 * t2952;
    let t2955 = t2262 * t338;
    (t2942, t2944, t2946, t2948, t2950, t2952, t2954, t2955)
}
