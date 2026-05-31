//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 744/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk744<F: Float>(t4574: F, t811: F, t1949: F, t3974: F, t5165: F, t1944: F, t2022: F, t4475: F, t2030: F, t4479: F, t3965: F, t4722: F, t784: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6748 = t4574 * t811;
    let t6749 = t6748 * t1949;
    let t6751 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t3974 * t6749;
    let t6752 = t5165 * t811;
    let t6753 = t6752 * t1944;
    let t6755 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t3974 * t6753;
    let t6756 = t4475 * t2022;
    let t6758 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t3974 * t6756;
    let t6759 = t4479 * t2030;
    let t6761 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t3965 * t6759;
    let t6762 = t4722 * t784;
    (t6748, t6749, t6751, t6752, t6753, t6755, t6756, t6758, t6759, t6761, t6762)
}
