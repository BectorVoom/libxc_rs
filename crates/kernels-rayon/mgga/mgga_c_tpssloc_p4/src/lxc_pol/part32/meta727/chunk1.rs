//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2354/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2354(t104729: f64, t104976: f64, t104977: f64, t104990: f64, t1458: f64, t19534: f64, t24932: f64, t27863: f64, t27888: f64, t33690: f64, t4072: f64, t5493: f64, t671: f64, t7266: f64, t96238: f64, t96659: f64, t96661: f64, t96663: f64, t96665: f64) -> f64 {
    let t104995 = 4.0_f64 * t104977 * t1458 + 2.0_f64 * t104990 * t671 + 4.0_f64 * t1458 * t96238 + 2.0_f64 * t19534 * t7266 + 2.0_f64 * t24932 * t5493 + 4.0_f64 * t27863 * t4072 + 2.0_f64 * t27888 * t5493 + 4.0_f64 * t33690 * t4072 + 2.0_f64 * t104729 + t104976 + t96659 + t96661 + t96663 + t96665;
    t104995
}
