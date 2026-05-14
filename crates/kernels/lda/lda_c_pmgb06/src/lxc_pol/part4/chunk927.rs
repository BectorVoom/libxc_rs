//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 927/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk927<F: Float>(t1435: F, t1540: F, t1426: F, t1592: F, t3238: F, t517: F, t1427: F, t3213: F, t1710: F, t431: F, t446: F, t2010: F, t1981: F, t500: F, t1417: F, t3223: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10255 = t1435 * t1540;
    let t10288 = t1426 * t1592;
    let t10293 = t3238 * t517;
    let t10316 = t3213 * t1427;
    let t10318 = t431 * t1710;
    let t10319 = t10318 * t446;
    let t10321 = t2010 * t446;
    let t10335 = t1981 * t500;
    let t10339 = t3223 * t1417;
    (t10255, t10288, t10293, t10316, t10318, t10319, t10321, t10335, t10339)
}
