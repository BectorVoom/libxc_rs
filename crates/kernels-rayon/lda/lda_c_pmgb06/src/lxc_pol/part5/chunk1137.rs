//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1137/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1137(t2002: f64, t6241: f64, t6245: f64, t20627: f64, t20632: f64, t20636: f64, t20641: f64, t20643: f64, t20646: f64, t20648: f64, t20651: f64, t20654: f64, t20656: f64) -> (f64, f64, f64) {
    let t20658 = t2002 * t6241 / 15.0_f64;
    let t20660 = t2002 * t6245 / 15.0_f64;
    let t20661 = t20627 + t20632 - t20636 + t20641 - t20643 - t20646 + t20648 + t20651 + t20654 + t20656 + t20658 + t20660;
    (t20658, t20660, t20661)
}
