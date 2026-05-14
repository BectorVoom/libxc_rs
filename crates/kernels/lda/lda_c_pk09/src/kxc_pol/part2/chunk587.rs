//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 587/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk587<F: Float>(t5068: F, t373: F, t4762: F, t332: F, t383: F, t4767: F, t1403: F, t5308: F, t1336: F, t1426: F, t1625: F, t1424: F, t1422: F, t387: F, t5141: F, t1421: F) -> (F, F, F, F, F, F, F, F) {
    let t5538 = 0.010056629776875343 * t5068;
    let t5542 = t4762 * t373;
    let t5544 = 0.018289183791044262 * t332 * t5542;
    let t5546 = 8.282336896725763 * t383 * t4767;
    let t5547 = t1403 * t5308;
    let t5549 = t1426 * t1336;
    let t5550 = t5549 * t1625;
    let t5554 = t1424 * t1424;
    let t5555 = 1.0 / t5554;
    let t5558 = t387 * t1422;
    let t5561 = 1.0 / t5141;
    let t5562 = t1421 * t5561;
    (t5538, t5544, t5546, t5547, t5550, t5555, t5558, t5562)
}
