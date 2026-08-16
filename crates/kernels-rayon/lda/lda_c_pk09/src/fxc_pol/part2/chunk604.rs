//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 604/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk604(t215: f64, t3065: f64, t68: f64, t221: f64, t3044: f64, t1142: f64, t1146: f64, t1228: f64, t1232: f64, t13: f64, t229: f64, t3040: f64, t3061: f64, t3080: f64, t600: f64, t604: f64, t606: f64, t9: f64) -> f64 {
    let t4960 = t215 * t3065 * t68;
    let t4966 = t3044 * t221;
    let t4977 = -0.044269133291355525_f64 * t1228 * t3044 * t215 + 0.021620978523742146_f64 * t229 * t1146 * t604 * t215 - 0.01479859893588926_f64 * t4960 * t3040 * t9 + 0.17758318723067112_f64 * t1232 * t3061 - 0.005365312962770623_f64 * t600 * t4966 + 5.2488911494204485e-05_f64 * t13 * t1142 * t221 - 0.007617107004393185_f64 * t3080 * t3040 * t221 + 0.011425660506589778_f64 * t606 * t4966;
    t4977
}
