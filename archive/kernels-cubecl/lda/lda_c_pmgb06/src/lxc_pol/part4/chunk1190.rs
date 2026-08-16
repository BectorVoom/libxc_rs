//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1190/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1190<F: Float>(t2002: F, t5226: F, t5242: F, t5245: F, t5248: F, t6275: F, t1902: F, t5187: F, t5254: F, t5257: F, t5261: F, t1916: F, t5305: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15703 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2002 * t5226;
    let t15705 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t2002 * t5242;
    let t15707 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2002 * t5245;
    let t15709 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t6275 * t5248;
    let t15711 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t5187 * t1902;
    let t15713 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t2002 * t5254;
    let t15715 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t2002 * t5257;
    let t15717 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t2002 * t5261;
    let t15719 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5305 * t1916;
    (t15703, t15705, t15707, t15709, t15711, t15713, t15715, t15717, t15719)
}
