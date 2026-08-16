//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1013/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1013(t325: f64, t6651: f64, t4606: f64, t6654: f64, t331: f64, t6824: f64, t6827: f64, t5021: f64, t6830: f64, t6818: f64, t6821: f64, t2140: f64, t5334: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16432 = t325 * t6651;
    let t16434 = t4606 * t6654;
    let t16439 = t331 * t6824;
    let t16441 = t331 * t6827;
    let t16445 = t5021 * t6830;
    let t16468 = t331 * t6818;
    let t16470 = t331 * t6821;
    let t16514 = t5334 * t2140;
    (t16432, t16434, t16439, t16441, t16445, t16468, t16470, t16514)
}
