//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 715/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk715<F: Float>(t1672: F, t2104: F, t472: F, t6601: F, t2000: F, t451: F, t6196: F, t6501: F, t6505: F, t6522: F, t6319: F, t6325: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7091 = t2104 * t1672;
    let t7098 = F::new(2.0) / F::new(27.0) * t472 * t6601;
    let t7102 = t451 * t2000;
    let t7103 = t7102 * t6196;
    let t7107 = F::cast_from(1.5323028051206833_f64) * t6501;
    let t7108 = F::cast_from(1.5323028051206833_f64) * t6505;
    let t7112 = F::cast_from(2.0430704068275776_f64) * t6522;
    let t7116 = F::cast_from(0.3056501876701794_f64) * t6319;
    let t7123 = F::cast_from(0.2037667917801196_f64) * t6325;
    (t7091, t7098, t7102, t7103, t7107, t7108, t7112, t7116, t7123)
}
