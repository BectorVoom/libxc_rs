//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 665/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk665<F: Float>(t173: F, t4008: F, t184: F, t199: F, t3638: F, t3625: F, t3627: F, t3629: F, t3631: F, t3635: F, t3641: F, t3644: F, t3646: F, t3649: F, t3652: F) -> (F, F, F, F, F) {
    let t4009 = t173 * t4008;
    let t4010 = t4009 * t184;
    let t4012 = F::new(2.0) / F::new(15.0) * t4010 * t199;
    let t4013 = F::cast_from(0.005877407407407408_f64) * t3638;
    let t4024 = t4013 + F::cast_from(0.002518888888888889_f64) * t3627 - F::cast_from(0.0012594444444444445_f64) * t3631 + F::cast_from(0.003778333333333333_f64) * t3646 - F::cast_from(0.0018891666666666666_f64) * t3629 + F::cast_from(0.002099074074074074_f64) * t3635 - F::cast_from(0.007556666666666666_f64) * t3649 + F::cast_from(0.003778333333333333_f64) * t3652 + F::new(0.011335) * t3641 - F::new(0.011335) * t3644 + F::cast_from(0.0018891666666666666_f64) * t3625;
    (t4009, t4010, t4012, t4013, t4024)
}
