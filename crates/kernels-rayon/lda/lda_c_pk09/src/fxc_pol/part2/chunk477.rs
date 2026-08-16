//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 477/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk477(t1330: f64, t2507: f64, t306: f64, t2143: f64, t372: f64, t1349: f64, t1354: f64, t1356: f64, t1358: f64, t1360: f64, t2502: f64, t2505: f64, t2542: f64, t2546: f64) -> (f64, f64, f64, f64, f64) {
    let t2636 = t2507 * t1330;
    let t2637 = t2636 * t306;
    let t2640 = t372 * t2143;
    let t2641 = t1349 * t2640;
    let t2648 = t1354 - 4.0_f64 * t2542 + t1356 + 4.0_f64 * t2546 + t1358 - 0.821419393556371_f64 * t2502 + t1360 + 0.821419393556371_f64 * t2505;
    (t2636, t2637, t2640, t2641, t2648)
}
