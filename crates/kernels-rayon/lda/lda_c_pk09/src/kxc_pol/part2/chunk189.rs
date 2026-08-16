//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 189/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk189(t613: f64, t617: f64, t626: f64, t636: f64, t653: f64, t186: f64, t187: f64, t183: f64, t10: f64, t34: f64, t39: f64, t41: f64) -> (f64, f64, f64, f64, f64) {
    let t655 = t613 + t617 + 0.9421211958699838_f64 * t626 + 0.9421211958699838_f64 * t636 - 0.9421211958699838_f64 * t653;
    let t658 = 1.0_f64 / t187 / t186;
    let t659 = t183 * t658;
    let t660 = t34 * t10;
    let t661 = t39 * t41;
    (t655, t658, t659, t660, t661)
}
