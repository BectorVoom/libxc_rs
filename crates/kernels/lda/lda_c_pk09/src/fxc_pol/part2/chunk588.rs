//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 588/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk588<F: Float>(t1397: F, t1413: F, t5558: F, t5562: F, t309: F, t310: F, t1223: F, t1284: F) -> (F, F) {
    let t5564 = t1397 * t5562 - 2.0 * t1413 * t5558;
    let t5566 = t309 * t310 * t5564;
    let t5569 = t1284 * t1223;
    (t5566, t5569)
}
