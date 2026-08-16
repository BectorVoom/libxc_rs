//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 948/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk948(t7335: f64, t5520: f64, t5522: f64, t5525: f64, t7352: f64, t7357: f64, t672: f64, t665: f64, t1861: f64, t2759: f64, t667: f64, t1867: f64, t2754: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7359 = 2.0_f64 / 3.0_f64 * t7335;
    let t7360 = -t5520 + 8.0_f64 / 9.0_f64 * t5522 - t5525 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t7357 - t7359 + t7352;
    let t7361 = t672 * t7360;
    let t7363 = t665 * t7360;
    let t7365 = t1861 * t2759;
    let t7366 = t7365 * t667;
    let t7368 = t2754 * t1867;
    (t7360, t7361, t7363, t7365, t7366, t7368)
}
