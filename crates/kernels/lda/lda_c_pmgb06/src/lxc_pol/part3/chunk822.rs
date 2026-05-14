//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 822/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk822<F: Float>(t3259: F, t458: F, t1435: F, t1540: F, t132: F, t3442: F, t435: F, t1447: F, t2980: F, t1423: F, t2949: F, t2866: F, t1426: F, t1592: F, t3238: F, t517: F) -> (F, F, F, F, F, F, F, F) {
    let t10247 = t3259 * t458;
    let t10255 = t1435 * t1540;
    let t10267 = t132 * t435 * t3442;
    let t10269 = t1447 * t2980;
    let t10273 = t1423 * t2949;
    let t10286 = t1423 * t2866;
    let t10288 = t1426 * t1592;
    let t10293 = t3238 * t517;
    (t10247, t10255, t10267, t10269, t10273, t10286, t10288, t10293)
}
