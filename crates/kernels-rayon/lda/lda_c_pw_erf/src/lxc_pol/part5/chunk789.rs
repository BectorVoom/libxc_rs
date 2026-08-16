//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 789/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk789(t1: f64, t2363: f64, t3: f64, t604: f64, t5039: f64, t5055: f64, t5057: f64, t5172: f64, t6751: f64, t6755: f64, t6758: f64, t6761: f64, t6765: f64, t6769: f64, t6773: f64, t6776: f64, t6778: f64, t6780: f64, t6782: f64) -> (f64, f64, f64) {
    let t7266 = t2363 * t1 * t3;
    let t7267 = t7266 * t604;
    let t7269 = -t6751 + t6755 - t6758 - t6761 - t6765 + t6769 - t6773 - t6776 - t6778 - t6780 + 0.10821041362364843_f64 * t7267 + t6782 + t5039 - t5055 - t5057 - t5172;
    (t7266, t7267, t7269)
}
