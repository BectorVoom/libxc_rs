//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 869/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk869<F: Float>(t242: F, t4120: F, t1159: F, t632: F, t1143: F, t695: F, t9196: F, t4100: F, t1203: F, t2929: F, t466: F, t10953: F, t148: F, t1198: F, t458: F, t116: F, t1191: F, t731: F, t732: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11217 = t4120 * t242;
    let t11219 = t1159 * t632;
    let t11222 = 1.0051538464260528 * t695 * t1143;
    let t11223 = t9196 * t242;
    let t11225 = t4100 * t632;
    let t11227 = t1203 * t1143;
    let t11229 = t466 * t2929;
    let t11232 = 0.0837628205355044 * t148 * t10953;
    let t11233 = t1198 * t1143;
    let t11236 = 0.3350512821420176 * t458 * t2929;
    let t11250 = 6.693920255418272 * t731 * t732 * t1191 * t116;
    (t11217, t11219, t11222, t11223, t11225, t11227, t11229, t11232, t11233, t11236, t11250)
}
