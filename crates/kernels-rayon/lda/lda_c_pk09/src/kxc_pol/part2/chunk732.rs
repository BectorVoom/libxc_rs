//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 732/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk732(t2417: f64, t4604: f64, t119: f64, t121: f64, t803: f64, t120: f64, t2250: f64, t623: f64, t891: f64) -> (f64, f64, f64, f64) {
    let t7583 = t2417 * t4604;
    let t7584 = t7583 * t119;
    let t7585 = t121 * t803;
    let t7586 = t120 * t7585;
    let t7589 = t2250 * t623;
    let t7590 = t891 * t7589;
    (t7584, t7586, t7589, t7590)
}
