//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 255/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk255(t352: f64, t359: f64, t360: f64, t368: f64, t63: f64, t775: f64, t777: f64, t780: f64, t381: f64, t69: f64) -> (f64, f64) {
    let t783 = -t352 - t775 - t359 - t360 * t777 / 2.0_f64 - t368 - 1.46904_f64 * t63 * t780;
    let t787 = -t352 - t775 - t381 - 1.724255_f64 * t69 * t777;
    (t783, t787)
}
