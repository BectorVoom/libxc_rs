//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1202/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1202(t12587: f64, t12591: f64, t12597: f64, t12602: f64, t12604: f64, t12608: f64, t12610: f64, t12612: f64, t12620: f64, t12622: f64, t12624: f64, t12626: f64, t12628: f64, t12630: f64, t12632: f64, t12636: f64, t12641: f64, t12643: f64, t12645: f64, t12648: f64, t12650: f64, t12653: f64, t12654: f64) -> (f64, f64) {
    let t14380 = t12587 - t12591 + t12597 - t12602 - t12604 - t12608 + t12610 - t12612 - t12620 + t12622 - t12624;
    let t14381 = -t12626 - t12628 - t12630 - t12632 + t12636 - t12641 - t12643 - t12645 - t12648 - t12650 - t12653 - t12654;
    (t14380, t14381)
}
