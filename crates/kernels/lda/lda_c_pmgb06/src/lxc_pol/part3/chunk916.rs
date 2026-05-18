//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 916/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk916<F: Float>(t1423: F, t2966: F, t1447: F, t2877: F, t3216: F, t464: F, t1387: F, t3220: F, t3260: F, t3031: F, t442: F, t3248: F, t517: F) -> (F, F, F, F, F, F, F) {
    let t10393 = t1423 * t2966;
    let t10403 = t1447 * t2877;
    let t10412 = t3216 * t464;
    let t10416 = t3220 * t1387;
    let t10431 = t3260 * t464;
    let t10439 = t442 * t3031;
    let t10445 = t3248 * t517;
    (t10393, t10403, t10412, t10416, t10431, t10439, t10445)
}
