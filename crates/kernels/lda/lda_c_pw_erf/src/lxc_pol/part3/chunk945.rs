//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 945/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk945<F: Float>(t1631: F, t4183: F, t1634: F, t474: F, t602: F, t1210: F, t1638: F, t603: F, t1639: F, t20: F, t3945: F, t4196: F) -> (F, F, F, F, F) {
    let t10690 = t1631 * t4183;
    let t10694 = F::cast_from(0.38474813732852775_f64) * t602 * t474 * t1634;
    let t10697 = F::cast_from(0.019878653761973935_f64) * t1638 * t1210 * t603;
    let t10699 = t3945 * t20 * t1639;
    let t10702 = t1631 * t4196;
    (t10690, t10694, t10697, t10699, t10702)
}
