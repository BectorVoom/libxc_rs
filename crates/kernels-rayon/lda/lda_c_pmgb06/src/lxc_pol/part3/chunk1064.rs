//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1064/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1064(t1560: f64, t5187: f64, t1447: f64, t4757: f64, t12620: f64, t12622: f64, t12624: f64, t12626: f64, t12628: f64, t12630: f64, t12632: f64, t12636: f64, t12641: f64) -> (f64, f64, f64) {
    let t12643 = 2.0_f64 / 15.0_f64 * t5187 * t1560;
    let t12644 = t1447 * t4757;
    let t12645 = 4.0_f64 / 45.0_f64 * t12644;
    let t12646 = -t12620 + t12622 - t12624 - t12626 - t12628 - t12630 - t12632 + t12636 - t12641 - t12643 - t12645;
    (t12643, t12645, t12646)
}
