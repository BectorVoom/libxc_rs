//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 604/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk604<F: Float>(t215: F, t3065: F, t68: F, t221: F, t3044: F, t1142: F, t1146: F, t1228: F, t1232: F, t13: F, t229: F, t3040: F, t3061: F, t3080: F, t600: F, t604: F, t606: F, t9: F) -> F {
    let t4960 = t215 * t3065 * t68;
    let t4966 = t3044 * t221;
    let t4977 = -F::new(0.044269133291355525) * t1228 * t3044 * t215 + F::new(0.021620978523742146) * t229 * t1146 * t604 * t215 - F::new(0.01479859893588926) * t4960 * t3040 * t9 + F::new(0.17758318723067112) * t1232 * t3061 - F::new(0.005365312962770623) * t600 * t4966 + F::new(5.2488911494204485e-05) * t13 * t1142 * t221 - F::new(0.007617107004393185) * t3080 * t3040 * t221 + F::new(0.011425660506589778) * t606 * t4966;
    t4977
}
