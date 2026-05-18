//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 960/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk960<F: Float>(t1435: F, t2580: F, t318: F, t9819: F, t1625: F, t1336: F, t2579: F, t2578: F, t305: F, t1632: F, t304: F, t10059: F, t328: F) -> (F, F, F, F, F, F) {
    let t10140 = t2580 * t1435;
    let t10142 = t318 * t9819;
    let t10143 = t10142 * t1625;
    let t10145 = t2579 * t1336;
    let t10146 = t10145 * t1625;
    let t10150 = t2578 * t305;
    let t10151 = t1632 * t10150;
    let t10154 = t304 * t9819;
    let t10155 = t10154 * t1625;
    let t10162 = t328 * t10059;
    (t10140, t10143, t10146, t10151, t10155, t10162)
}
