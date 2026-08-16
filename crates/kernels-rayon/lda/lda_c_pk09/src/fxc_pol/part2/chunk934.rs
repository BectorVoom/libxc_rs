//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 934/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk934(t1216: f64, t2513: f64, t332: f64, t9770: f64, t10: f64, t5420: f64, t2517: f64, t1513: f64, t9851: f64, t1494: f64, t2611: f64, t382: f64, t9739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9885 = t1216 * t2513;
    let t9887 = t332 * t9770;
    let t9889 = t5420 * t10;
    let t9890 = t9889 * t2517;
    let t9892 = t1513 * t9851;
    let t9894 = t1494 * t2611;
    let t9896 = t382 * t9739;
    (t9885, t9887, t9890, t9892, t9894, t9896)
}
