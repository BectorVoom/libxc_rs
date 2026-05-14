//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 991/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk991<F: Float>(t1423: F, t5261: F, t5257: F, t5242: F, t5245: F, t5273: F, t1447: F, t5277: F, t1966: F, t3031: F, t5333: F, t5451: F, t5454: F, t5458: F, t5499: F, t1920: F, t3226: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13761 = t1423 * t5261;
    let t13763 = t1423 * t5257;
    let t13768 = t1423 * t5242;
    let t13770 = t1423 * t5245;
    let t13775 = t1423 * t5273;
    let t13782 = t1447 * t5277;
    let t13788 = t1966 * t3031;
    let t13807 = t1447 * t5333;
    let t13836 = t1447 * t5451;
    let t13838 = t1447 * t5454;
    let t13840 = t5499 * t5458;
    let t13842 = t3226 * t1920;
    (t13761, t13763, t13768, t13770, t13775, t13782, t13788, t13807, t13836, t13838, t13840, t13842)
}
