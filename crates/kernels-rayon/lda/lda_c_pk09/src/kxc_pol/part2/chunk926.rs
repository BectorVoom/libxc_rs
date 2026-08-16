//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 926/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk926(t1439: f64, t9796: f64, t1449: f64, t5040: f64, t5043: f64, t5046: f64, t5047: f64, t5056: f64, t5069: f64, t5071: f64, t9623: f64, t9628: f64, t9631: f64, t9635: f64, t9742: f64, t9746: f64, t9750: f64, t9753: f64, t9756: f64) -> (f64, f64) {
    let t9797 = t1439 * t9796;
    let t9798 = t9797 * t1449;
    let t9814 = t5040 - 2.0_f64 * t5043 + t5046 + 2.0_f64 * t5047 - 2.0_f64 * t9623 + 4.0_f64 * t9628 - 2.0_f64 / 3.0_f64 * t9631 - 2.0_f64 * t9635 - 2.0_f64 * t9742 - 2.0_f64 / 3.0_f64 * t5056 - t5069 + 2.0_f64 / 3.0_f64 * t5071 + 2.0_f64 * t9746 - 2.0_f64 * t9750 + 2.0_f64 / 3.0_f64 * t9753 + 2.0_f64 * t9756;
    (t9798, t9814)
}
