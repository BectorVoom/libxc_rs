//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 674/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk674(t3113: f64, t378: f64, t1201: f64, t881: f64, t1209: f64, t889: f64, t2175: f64, t2224: f64, t2303: f64, t2308: f64, t3017: f64, t3028: f64, t3042: f64, t3047: f64, t3053: f64, t3055: f64, t3059: f64, t3063: f64, t3067: f64) -> (f64, f64, f64, f64) {
    let t3114 = t3113 * t378;
    let t3116 = t1201 * t881;
    let t3121 = t1209 * t889;
    let t3135 = -0.1294625e1_f64 * t3042 + 0.258925e1_f64 * t3047 + t2303 - 0.301925e0_f64 * t2175 - 0.301925e0_f64 * t3017 + 0.905775e0_f64 * t3028 + 0.82524375e-1_f64 * t3053 + 0.16504875e0_f64 * t3055 + t2308 - 0.16557e0_f64 * t2224 - 0.16557e0_f64 * t3059 + 0.248355e0_f64 * t3063 + 0.248355e0_f64 * t3067;
    (t3114, t3116, t3121, t3135)
}
