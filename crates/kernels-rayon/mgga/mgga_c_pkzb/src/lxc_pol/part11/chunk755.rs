//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 755/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk755(t2277: f64, t861: f64, t356: f64, t2280: f64, t364: f64, t2317: f64, t877: f64, t2278: f64, t858: f64, t361: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6287 = 1.0_f64 / t2277 / t861;
    let t6288 = t356 * t6287;
    let t6290 = 1.0_f64 / t2280 / t364;
    let t6300 = t877 * t2317;
    let t6308 = t858 * t2278;
    let t6312 = 1.0_f64 / t2277 / t361;
    let t6313 = t356 * t6312;
    (t6287, t6288, t6290, t6300, t6308, t6312, t6313)
}
