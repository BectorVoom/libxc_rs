//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 901/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk901<F: Float>(t1447: F, t4762: F, t1420: F, t4767: F, t1594: F, t1966: F, t2064: F, t3031: F, t439: F, t1423: F, t5198: F, t4766: F, t5197: F, t1430: F, t4779: F, t1435: F, t1872: F) -> (F, F, F, F, F, F, F) {
    let t12075 = t1447 * t4762;
    let t12076 = 2.0 / 5.0 * t12075;
    let t12078 = 3.0 / 5.0 * t1420 * t4767;
    let t12083 = 3.0 / 5.0 * t439 * t1966 * t3031 * t2064 * t1594;
    let t12084 = t1423 * t5198;
    let t12085 = 4.0 / 15.0 * t12084;
    let t12088 = 3.0 / 5.0 * t439 * t5197 * t4766;
    let t12091 = t439 * t4779 * t1430 / 15.0;
    let t12092 = t1435 * t1872;
    (t12076, t12078, t12083, t12085, t12088, t12091, t12092)
}
