//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1126/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1126(t4283: f64, t7061: f64, t986: f64, t1005: f64, t4311: f64, t1434: f64, t3579: f64, t4327: f64, t4324: f64, t11016: f64, t11018: f64, t11021: f64, t11024: f64, t11027: f64, t2577: f64, t2599: f64, t3532: f64, t3565: f64, t3584: f64, t7059: f64, t7109: f64, t9245: f64, t9248: f64, t9255: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11116 = t4283 * t7061;
    let t11117 = t11116 * t986;
    let t11124 = t4311 * t1005;
    let t11127 = t1434 * t3579;
    let t11130 = t4327 * t1005;
    let t11133 = t4324 * t1005;
    let t11138 = 0.2069040516770936012e4_f64 * t7059 * t11117 - 0.23392894490538584828e1_f64 * t9255 * t3565 + 0.34631718211362927517e2_f64 * t9248 * t3584 + 0.35089341735807877242e1_f64 * t2599 * t11124 - 0.23392894490538584828e1_f64 * t2577 * t11127 - 0.10389515463408878255e3_f64 * t7109 * t11130 - 0.11696447245269292414e1_f64 * t2577 * t11133 - 4.0_f64 * t9245 * t3532 + t11016 - t11018 - t11021 + t11024 + t11027;
    (t11116, t11117, t11124, t11127, t11130, t11133, t11138)
}
