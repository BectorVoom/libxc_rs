//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 694/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk694<F: Float>(t6464: F, t2000: F, t6488: F, t430: F, t4990: F, t10: F, t132: F, t4993: F, t93: F, t1468: F, t429: F) -> (F, F, F, F, F, F, F) {
    let t6563 = F::cast_from(0.14222222222222222_f64) * t6464;
    let t6574 = t2000 * t6488;
    let t6575 = F::cast_from(14.71989892086604_f64) * t6574;
    let t6576 = t4990 * t430;
    let t6577 = t6576 * t10;
    let t6578 = t132 * t4993;
    let t6579 = t93 * t6578;
    let t6580 = t6577 * t6579;
    let t6581 = F::cast_from(16.20073542583857_f64) * t6580;
    let t6586 = t1468 * t429;
    (t6563, t6574, t6575, t6579, t6580, t6581, t6586)
}
