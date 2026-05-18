//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 889/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk889<F: Float>(t3323: F, t3326: F, t3424: F, t3426: F, t3428: F, t4245: F, t4252: F, t4254: F, t7870: F, t7875: F, t7879: F, t7884: F, t7888: F) -> F {
    let t9375 = F::new(0.2037667917801196) * t3323 + F::new(0.2037667917801196) * t3326 + t4245 + F::new(9.1938168307241) * t7870 - F::new(9.1938168307241) * t7875 + F::new(9.1938168307241) * t7879 - F::new(9.1938168307241) * t7884 + F::new(9.1938168307241) * t7888 + F::new(6.129211220482733) * t3424 + F::new(6.129211220482733) * t3426 - F::new(6.129211220482733) * t3428 + t4252 + t4254;
    t9375
}
