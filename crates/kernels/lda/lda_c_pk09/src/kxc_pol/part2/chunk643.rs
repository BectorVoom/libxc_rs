//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 643/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk643<F: Float>(t1403: F, t5308: F, t1336: F, t1426: F, t1625: F, t1424: F, t1422: F, t387: F, t5141: F, t1421: F, t1397: F, t1413: F) -> (F, F, F, F) {
    let t5547 = t1403 * t5308;
    let t5549 = t1426 * t1336;
    let t5550 = t5549 * t1625;
    let t5554 = t1424 * t1424;
    let t5555 = F::cast_from(1.0_f64) / t5554;
    let t5558 = t387 * t1422;
    let t5561 = F::cast_from(1.0_f64) / t5141;
    let t5562 = t1421 * t5561;
    let t5564 = t1397 * t5562 - F::cast_from(2.0_f64) * t1413 * t5558;
    (t5547, t5550, t5555, t5564)
}
