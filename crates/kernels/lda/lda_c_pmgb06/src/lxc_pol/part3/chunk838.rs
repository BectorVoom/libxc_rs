//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 838/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk838<F: Float>(t1186: F, t1770: F, t4239: F, t1099: F, t33: F, t419: F, t83: F, t31: F, t4001: F, t122: F, t302: F, t1755: F, t1773: F) -> (F, F, F, F, F, F) {
    let t8081 = F::cast_from(0.0006558687695417436_f64) * t4239 * t1186 * t1770;
    let t8083 = F::new(1.0) / t33 / t1099;
    let t8085 = t8083 * t83 * t419;
    let t8087 = F::cast_from(0.0012955432484775182_f64) * t8085 * t1770;
    let t8088 = t31 * t4001;
    let t8091 = F::cast_from(0.9106331049773876_f64) * t122 * t8088 * t302;
    let t8092 = t1773 * t1755;
    (t8081, t8085, t8087, t8088, t8091, t8092)
}
