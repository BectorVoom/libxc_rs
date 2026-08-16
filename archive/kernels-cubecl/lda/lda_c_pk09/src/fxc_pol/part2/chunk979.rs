//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 979/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk979<F: Float>(t10454: F, t1625: F, t10020: F, t1285: F, t2665: F, t5239: F, t306: F, t1277: F, t309: F, t310: F, t1382: F, t2487: F) -> (F, F, F, F, F) {
    let t10455 = t10454 * t1625;
    let t10459 = t1285 * t10020;
    let t10465 = t2665 * t5239;
    let t10466 = t10465 * t306;
    let t10468 = t309 * t310 * t1277;
    let t10471 = t1382 * t2487;
    (t10455, t10459, t10466, t10468, t10471)
}
