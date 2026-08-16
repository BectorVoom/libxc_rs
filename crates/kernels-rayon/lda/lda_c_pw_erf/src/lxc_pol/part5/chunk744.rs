//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 744/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk744(t4574: f64, t811: f64, t1949: f64, t3974: f64, t5165: f64, t1944: f64, t2022: f64, t4475: f64, t2030: f64, t4479: f64, t3965: f64, t4722: f64, t784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6748 = t4574 * t811;
    let t6749 = t6748 * t1949;
    let t6751 = 32.0_f64 / 45.0_f64 * t3974 * t6749;
    let t6752 = t5165 * t811;
    let t6753 = t6752 * t1944;
    let t6755 = 16.0_f64 / 27.0_f64 * t3974 * t6753;
    let t6756 = t4475 * t2022;
    let t6758 = 16.0_f64 / 45.0_f64 * t3974 * t6756;
    let t6759 = t4479 * t2030;
    let t6761 = 16.0_f64 / 45.0_f64 * t3965 * t6759;
    let t6762 = t4722 * t784;
    (t6748, t6749, t6751, t6752, t6753, t6755, t6756, t6758, t6759, t6761, t6762)
}
