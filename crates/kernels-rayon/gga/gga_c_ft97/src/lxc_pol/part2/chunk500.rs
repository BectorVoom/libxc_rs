//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 500/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk500(t299: f64, t2956: f64, t332: f64, t113: f64, t909: f64, t505: f64, t910: f64, t1934: f64, t2900: f64, t2904: f64, t333: f64, t5: f64, t886: f64, t889: f64, t911: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t300 = 10000000.0_f64 <= t299;
    let t2957 = t2956 * t332;
    let t2958 = t2957 * t113;
    let t2961 = t909 * t909;
    let t2962 = t2961 * t332;
    let t2963 = t2962 * t113;
    let t2966 = t910 * t505;
    let t2973 = piecewise3(t300, 0.0_f64, t5 * t2900 * t113 / 4.0_f64 + t2904 * t911 / 2.0_f64 + t5 * t886 * t505 / 2.0_f64 + t889 * t2958 / 4.0_f64 + t889 * t2963 / 4.0_f64 + t889 * t2966 / 2.0_f64 + t5 * t333 * t1934 / 4.0_f64);
    (t2957, t2958, t2961, t2962, t2963, t2966, t2973)
}
