//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 963/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk963<F: Float>(t2606: F, t6026: F, t1625: F, t2595: F, t5777: F, t10020: F, t5711: F, t327: F, t332: F, t5829: F, t1215: F, t2579: F) -> (F, F, F, F, F, F) {
    let t10198 = t2606 * t6026;
    let t10199 = t10198 * t1625;
    let t10201 = t2595 * t5777;
    let t10204 = t5711 * t10020;
    let t10206 = t327 * t332;
    let t10209 = t5829 * t10020;
    let t10216 = t2579 * t1215;
    (t10199, t10201, t10204, t10206, t10209, t10216)
}
