//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 823/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk823<F: Float>(t1427: F, t3213: F, t1710: F, t431: F, t446: F, t2010: F, t1447: F, t3285: F, t1981: F, t500: F, t1417: F, t3226: F, t3223: F, t1166: F, t1696: F, t208: F, t213: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10316 = t3213 * t1427;
    let t10318 = t431 * t1710;
    let t10319 = t10318 * t446;
    let t10321 = t2010 * t446;
    let t10333 = t1447 * t3285;
    let t10335 = t1981 * t500;
    let t10337 = t3226 * t1417;
    let t10339 = t3223 * t1417;
    let t10343 = t1166 * t1696 * t208 * t213;
    (t10316, t10318, t10319, t10321, t10333, t10335, t10337, t10339, t10343)
}
