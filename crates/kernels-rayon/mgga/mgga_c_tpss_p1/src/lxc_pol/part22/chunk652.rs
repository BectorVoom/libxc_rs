//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 652/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk652(t1091: f64, t3009: f64, t1081: f64, t2973: f64, t2975: f64, t1089: f64, t1072: f64, t2993: f64, t2998: f64, t3001: f64, t215: f64, t442: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3011 = 0.11696447245269292414e1_f64 * t3009 * t1091;
    let t3013 = t2973 * t2975 * t1081;
    let t3015 = 0.11696447245269292414e1_f64 * t1089 * t3013;
    let t3017 = t1072 * t2993 * t1081;
    let t3019 = 0.5848223622634646207e0_f64 * t1089 * t3017;
    let t3020 = t2998 * t2975;
    let t3021 = t3020 * t3001;
    let t3023 = 0.17315859105681463759e2_f64 * t1089 * t3021;
    let t3025 = t215 * t671 * t442;
    (t3011, t3013, t3015, t3017, t3019, t3021, t3023, t3025)
}
