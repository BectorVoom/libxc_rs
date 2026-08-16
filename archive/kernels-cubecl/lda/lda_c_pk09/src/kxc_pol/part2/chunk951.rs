//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 951/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk951<F: Float>(t2595: F, t5687: F, t1625: F, t1240: F, t2512: F, t333: F) -> (F, F) {
    let t10016 = t2595 * t5687;
    let t10017 = t10016 * t1625;
    let t10019 = t2512 * t1240;
    let t10020 = t333 * t10019;
    (t10017, t10020)
}
