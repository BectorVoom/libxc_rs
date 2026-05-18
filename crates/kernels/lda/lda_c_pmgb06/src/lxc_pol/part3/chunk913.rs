//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 913/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk913<F: Float>(t1447: F, t2980: F, t1423: F, t2949: F, t2866: F, t1426: F, t1592: F, t3238: F, t517: F, t1427: F, t3213: F, t1710: F, t431: F) -> (F, F, F, F, F, F, F) {
    let t10269 = t1447 * t2980;
    let t10273 = t1423 * t2949;
    let t10286 = t1423 * t2866;
    let t10288 = t1426 * t1592;
    let t10293 = t3238 * t517;
    let t10316 = t3213 * t1427;
    let t10318 = t431 * t1710;
    (t10269, t10273, t10286, t10288, t10293, t10316, t10318)
}
