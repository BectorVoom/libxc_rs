//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 889/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk889<F: Float>(t3323: F, t3326: F, t3424: F, t3426: F, t3428: F, t4245: F, t4252: F, t4254: F, t7870: F, t7875: F, t7879: F, t7884: F, t7888: F) -> F {
    let t9375 = F::cast_from(0.2037667917801196_f64) * t3323 + F::cast_from(0.2037667917801196_f64) * t3326 + t4245 + F::cast_from(9.1938168307241_f64) * t7870 - F::cast_from(9.1938168307241_f64) * t7875 + F::cast_from(9.1938168307241_f64) * t7879 - F::cast_from(9.1938168307241_f64) * t7884 + F::cast_from(9.1938168307241_f64) * t7888 + F::cast_from(6.129211220482733_f64) * t3424 + F::cast_from(6.129211220482733_f64) * t3426 - F::cast_from(6.129211220482733_f64) * t3428 + t4252 + t4254;
    t9375
}
