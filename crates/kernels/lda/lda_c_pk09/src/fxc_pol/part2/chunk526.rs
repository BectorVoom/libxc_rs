//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 526/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk526<F: Float>(t3194: F, t839: F, t2974: F, t1067: F, t704: F, t132: F, t650: F, t609: F, t755: F, t120: F) -> (F, F, F, F, F) {
    let t3195 = t839 * t3194;
    let t3201 = t839 * t2974;
    let t3203 = t704 * t1067;
    let t3213 = t132 * t650;
    let t3222 = t755 * t609;
    let t3223 = t120 * t3222;
    (t3195, t3201, t3203, t3213, t3223)
}
