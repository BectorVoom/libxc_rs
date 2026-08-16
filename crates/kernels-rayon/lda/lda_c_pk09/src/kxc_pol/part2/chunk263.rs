//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 263/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk263(t1180: f64, t1185: f64, t1153: f64, t1164: f64, t1175: f64, t253: f64, t275: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t1186 = t1180 * t1185;
    let t1189 = t1153 - t1164 + 1.28_f64 * t253 * t1175 - 1.28_f64 * t253 * t1186;
    let t1190 = t275 * t1189;
    let t1191 = f64::ln(zeta_threshold);
    (t1186, t1189, t1190, t1191)
}
