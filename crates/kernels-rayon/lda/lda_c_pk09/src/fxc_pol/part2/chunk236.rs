//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 236/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk236(t721: f64, t957: f64, t612: f64, t616: f64, t626: f64, t636: f64, t653: f64, t82: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t959 = 2.427516195194328_f64 * t957 * t721;
    let t966 = 11.879313099038017_f64 * t612;
    let t967 = 7.919542066025344_f64 * t616;
    let t971 = t966 + t967 + 11.879313099038017_f64 * t626 + 11.879313099038017_f64 * t636 - 11.879313099038017_f64 * t653;
    let t972 = t82 * t82;
    let t973 = t972 + 1.0_f64;
    let t974 = 1.0_f64 / t973;
    let t975 = t971 * t974;
    (t959, t966, t967, t971, t973, t974, t975)
}
