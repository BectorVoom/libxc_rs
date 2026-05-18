//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 591/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk591<F: Float>(t178: F, t2971: F, t3190: F, t3767: F, t188: F, t733: F, t3743: F, t3745: F, t609: F, t891: F, t568: F, t948: F) -> (F, F, F, F, F) {
    let t4581 = t178 * t2971;
    let t4584 = t3767 * t3190;
    let t4586 = t188 * t733;
    let t4587 = t4586 * t3743;
    let t4589 = t891 * t3745 * t609;
    let t4590 = t4587 * t4589;
    let t4594 = t568 * t948;
    (t4581, t4584, t4587, t4590, t4594)
}
