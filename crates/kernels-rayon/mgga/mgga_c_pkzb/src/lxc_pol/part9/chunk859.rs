//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 859/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk859(t6224: f64, t881: f64, t890: f64, t898: f64, t2316: f64, t880: f64) -> (f64, f64, f64) {
    let t6226 = t881 * t6224 * t890;
    let t6228 = 0.5848223622634646207e0_f64 * t898 * t6226;
    let t6230 = 1.0_f64 / t2316 / t880;
    (t6226, t6228, t6230)
}
