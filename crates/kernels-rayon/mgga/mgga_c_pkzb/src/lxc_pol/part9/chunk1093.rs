//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1093/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1093(t17928: f64, t2018: f64, t197: f64, t2021: f64, t294: f64, t2029: f64, t750: f64, t2096: f64, t5940: f64, t148: f64, t616: f64, t757: f64, t762: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17929 = t17928 * t2018;
    let t17930 = t17929 * t197;
    let t17931 = t2021 * t2021;
    let t17932 = 1.0_f64 / t17931;
    let t17933 = t294 * t17932;
    let t17938 = t2029 * t2029;
    let t17945 = t17928 * t750;
    let t17946 = t17945 * t197;
    let t17953 = t2096 * t5940;
    let t17955 = t616 * t148;
    let t17957 = t757 * t17955 * t762;
    (t17929, t17930, t17932, t17933, t17938, t17945, t17946, t17953, t17955, t17957)
}
