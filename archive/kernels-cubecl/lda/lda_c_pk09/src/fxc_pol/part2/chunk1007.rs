//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1007/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1007<F: Float>(t10432: F, t10510: F, t10584: F, t10602: F, t10621: F, t10697: F, t10735: F, t10771: F, t10822: F, t10852: F, t10915: F, t10928: F, t9763: F, t9825: F, t9881: F, t9915: F) -> F {
    let t10932 = t9763 + t9825 + t9881 + t9915 + t10432 + t10510 + t10584 + t10602 + t10621 + t10697 + t10735 + t10771 + t10822 + t10852 + t10915 + t10928;
    t10932
}
