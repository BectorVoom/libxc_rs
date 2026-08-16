//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 649/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk649(t2900: f64, t2923: f64, t302: f64, t1066: f64, t759: f64, t761: f64, t2105: f64, t179: f64, t2068: f64, t299: f64, t197: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2924 = t2900 * t2923;
    let t2925 = t302 * t2924;
    let t2931 = t1066 * t759;
    let t2932 = t2931 * t761;
    let t2933 = t2105 * t2932;
    let t2939 = t179 * t2068 * t1066;
    let t2940 = t299 * t2939;
    let t2942 = t290 * t197;
    (t2924, t2925, t2932, t2933, t2939, t2940, t2942)
}
