//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1260/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1260(t6269: f64, t8016: f64, t898: f64, t2332: f64, t8028: f64, t3152: f64, t6279: f64, t2328: f64, t8303: f64, t3147: f64, t6124: f64, t2295: f64, t8098: f64, t891: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22162 = 0.14035736694323150897e2_f64 * t898 * t8016 * t6269;
    let t22164 = 0.35089341735807877242e1_f64 * t8028 * t2332;
    let t22167 = 0.11696447245269292414e1_f64 * t898 * t3152 * t6279;
    let t22169 = 0.10526802520742363173e2_f64 * t2328 * t8303;
    let t22171 = 0.10389515463408878255e3_f64 * t3147 * t6124;
    let t22175 = 0.35089341735807877242e1_f64 * t898 * t2295 * t8098 * t891;
    (t22162, t22164, t22167, t22169, t22171, t22175)
}
