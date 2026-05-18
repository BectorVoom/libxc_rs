//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 646/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk646<F: Float>(t1625: F, t5584: F, t1285: F, t5308: F, t1303: F, t1336: F, t360: F, t4767: F, t1382: F, t1469: F, t1475: F, t1214: F, t1471: F) -> (F, F, F, F, F, F, F) {
    let t5585 = t5584 * t1625;
    let t5587 = t1285 * t5308;
    let t5589 = t1303 * t1336;
    let t5590 = t5589 * t1625;
    let t5593 = F::new(8.661855010924512) * t360 * t4767;
    let t5594 = t1382 * t1336;
    let t5595 = t5594 * t1625;
    let t5603 = t1469 * t1475;
    let t5604 = t1471 * t1214;
    (t5585, t5587, t5590, t5593, t5595, t5603, t5604)
}
