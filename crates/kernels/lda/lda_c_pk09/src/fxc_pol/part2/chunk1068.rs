//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1068/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1068<F: Float>(t11551: F, t11577: F, t447: F, t452: F, t1971: F, t2846: F, t2794: F, t7284: F, t7286: F, t446: F, t95: F, t1815: F) -> (F, F, F, F, F) {
    let t11578 = t11551 + t11577;
    let t11579 = t447 * t11578;
    let t11580 = t11579 * t452;
    let t11583 = t2846 * t1971;
    let t11586 = t2794 * t7284;
    let t11587 = t11586 * t7286;
    let t11588 = t95 * t446;
    let t11589 = t11588 * t1815;
    (t11580, t11583, t11586, t11587, t11589)
}
