//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 553/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk553<F: Float>(t1185: F, t1207: F, t416: F, t1471: F, t747: F, t1525: F, t1435: F, t1546: F, t215: F, t3065: F, t68: F, t221: F, t3044: F, t1142: F, t1146: F, t1228: F, t1232: F, t13: F, t229: F, t3040: F, t3061: F, t3080: F, t600: F, t604: F, t606: F, t9: F) -> (F, F, F, F, F, F, F) {
    let t4917 = t1207 * t1185;
    let t4926 = 1.0 / t416;
    let t4943 = t747 * t1471;
    let t4944 = t1525 * t4943;
    let t4945 = 7.200326855928252 * t4944;
    let t4950 = t1546 * t1435;
    let t4960 = t215 * t3065 * t68;
    let t4966 = t3044 * t221;
    let t4977 = -0.044269133291355525 * t1228 * t3044 * t215 + 0.021620978523742146 * t229 * t1146 * t604 * t215 - 0.01479859893588926 * t4960 * t3040 * t9 + 0.17758318723067112 * t1232 * t3061 - 0.005365312962770623 * t600 * t4966 + 5.2488911494204485e-05 * t13 * t1142 * t221 - 0.007617107004393185 * t3080 * t3040 * t221 + 0.011425660506589778 * t606 * t4966;
    (t4917, t4926, t4943, t4944, t4945, t4950, t4977)
}
