//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 905/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk905<F: Float>(t10255: F, t153: F, t1859: F, t439: F, t4659: F, t5253: F, t10247: F, t4645: F, t2010: F, t4655: F, t1858: F, t3115: F, t1901: F, t1074: F, t4654: F, t10203: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12135 = t439 * t10255 * t153 * t1859 / 9.0;
    let t12138 = t439 * t5253 * t4659 / 9.0;
    let t12139 = t10247 * t153;
    let t12142 = 8.0 / 27.0 * t439 * t12139 * t4645;
    let t12145 = 4.0 / 9.0 * t2010 * t5253 * t4655;
    let t12146 = t1858 * t3115;
    let t12149 = t439 * t1901 * t12146 / 27.0;
    let t12150 = t4654 * t1074;
    let t12153 = 2.0 / 9.0 * t2010 * t1901 * t12150;
    let t12154 = t10203 * t153;
    (t12135, t12138, t12142, t12145, t12146, t12149, t12150, t12153, t12154)
}
