//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1092/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1092<F: Float>(t9003: F, t9017: F, t102: F, t120: F, t20283: F, t19571: F, t19574: F, t19577: F, t127: F, t19580: F, t19584: F, t19590: F, t19593: F, t19604: F, t19614: F, t436: F) -> (F, F, F, F, F, F, F) {
    let t20318 = F::cast_from(1.5156425925925925_f64) * t9003;
    let t20319 = F::cast_from(1.2991222222222223_f64) * t9017;
    let t20324 = F::cast_from(2.923025_f64) * t102 * t120 * t20283;
    let t20328 = F::cast_from(1.9486833333333333_f64) * t19571;
    let t20329 = F::cast_from(0.9743416666666667_f64) * t19574;
    let t20330 = F::cast_from(1.4615125_f64) * t19577;
    let t20337 = -t20324 - F::cast_from(1.46904_f64) * t127 * t436 * t20283 - t20328 + t20329 + t20330 + F::cast_from(44.0712_f64) * t19580 - F::cast_from(17.62848_f64) * t19584 + F::cast_from(6.0_f64) * t19590 - F::cast_from(3.0_f64) * t19593 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t19604 - F::cast_from(8.81424_f64) * t19614;
    (t20318, t20319, t20324, t20328, t20329, t20330, t20337)
}
