//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 853/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk853<F: Float>(t3323: F, t3326: F, t3424: F, t3426: F, t3428: F, t3643: F, t3650: F, t3652: F, t7870: F, t7875: F, t7879: F, t7884: F, t7888: F) -> F {
    let t8797 = F::new(0.4266666666666667) * t3323 + F::new(0.4266666666666667) * t3326 + t3643 + F::new(19.250905149166083) * t7870 - F::new(19.250905149166083) * t7875 + F::new(19.250905149166083) * t7879 - F::new(19.250905149166083) * t7884 + F::new(19.250905149166083) * t7888 + F::new(12.833936766110723) * t3424 + F::new(12.833936766110723) * t3426 - F::new(12.833936766110723) * t3428 + t3650 + t3652;
    t8797
}
