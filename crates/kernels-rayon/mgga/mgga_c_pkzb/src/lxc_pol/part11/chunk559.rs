//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 559/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk559(t158: f64, t2956: f64, t1143: f64, t2118: f64, t799: f64, t1123: f64, t306: f64, t2019: f64, t2029: f64, t2126: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2957 = t2956 * t158;
    let t2964 = t2118 * t1143;
    let t2965 = t2964 * t799;
    let t2968 = t306 * t1123;
    let t2969 = t2019 * t2968;
    let t2970 = t2029 * t2126;
    (t2957, t2964, t2965, t2968, t2969, t2970)
}
