//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 652/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk652<F: Float>(t4999: F, t5013: F, t1519: F, t327: F, t5308: F, t5022: F, t1475: F, t1506: F, t1214: F, t1610: F, t93: F, t5039: F) -> (F, F, F, F, F, F, F, F) {
    let t5707 = F::cast_from(0.09983749558483038_f64) * t4999;
    let t5710 = F::cast_from(0.29951248675449116_f64) * t5013;
    let t5711 = t327 * t1519;
    let t5712 = t5711 * t5308;
    let t5714 = F::cast_from(0.020557162358903314_f64) * t5022;
    let t5716 = t1506 * t1475;
    let t5717 = t1610 * t1214;
    let t5718 = t93 * t5717;
    let t5731 = F::cast_from(11.879313099038017_f64) * t5039;
    (t5707, t5710, t5711, t5712, t5714, t5716, t5718, t5731)
}
