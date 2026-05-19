//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 179/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk179<F: Float>(t616: F, t47: F, t42: F, t560: F) -> (F, F, F, F) {
    let t617 = F::cast_from(0.6280807972466558_f64) * t616;
    let t618 = t47 * t47;
    let t619 = F::new(1.0) / t618;
    let t620 = t42 - t560;
    (t617, t618, t619, t620)
}
