//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 544/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk544(t322: f64, t2940: f64, t1348: f64, t2983: f64, t1338: f64, t2952: f64, t2954: f64, t2982: f64, t352: f64, t855: f64, t2464: f64, t2486: f64, t889: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t332 = 0.25e1_f64 < t322;
    let t2987 = piecewise3(t332, t2940, 0.0_f64);
    let t2991 = t1348 * t2983;
    let t2995 = piecewise5(t323, t2952 + t2954, t331, t2982, -0.21e1_f64 * t1338 * t2983 * t352 - 0.105e1_f64 * t855 * t2987 * t352 - 0.1575e1_f64 * t2991 * t352);
    let t2997 = 0.36622894612013090108e-3_f64 * t2464;
    let t2998 = 8.0_f64 * t2486;
    let t2999 = t889 * t889;
    (t2987, t2991, t2995, t2997, t2998, t2999)
}
