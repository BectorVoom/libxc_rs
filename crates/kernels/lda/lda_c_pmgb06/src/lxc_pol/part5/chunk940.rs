//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 940/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk940<F: Float>(t432: F, t4836: F, t1696: F, t1798: F, t208: F, t213: F, t4075: F, t794: F, t5374: F, t588: F, t97: F, t3247: F, t842: F) -> (F, F, F, F, F) {
    let t13439 = t432 * t4836;
    let t13440 = t13439 / F::new(45.0);
    let t13444 = t1798 * t1696 * t208 * t213;
    let t13447 = t794 * t4075 * t208 * t213;
    let t13450 = t5374 * t97 * t588;
    let t13483 = t3247 * t842;
    (t13440, t13444, t13447, t13450, t13483)
}
