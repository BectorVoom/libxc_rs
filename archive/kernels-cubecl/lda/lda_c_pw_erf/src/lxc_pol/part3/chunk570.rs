//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 570/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk570<F: Float>(t1010: F, t155: F, t1953: F, t2061: F, t2717: F, t2720: F, t2723: F, t2728: F, t2730: F, t2732: F, t371: F, t363: F, t987: F) -> (F, F, F, F) {
    let t3046 = t155 * t1010;
    let t3058 = -F::cast_from(4.7063_f64) * t2717 + F::cast_from(3.1375333333333333_f64) * t2720 - F::cast_from(3.6604555555555556_f64) * t2723 - F::cast_from(1.6068111111111112_f64) * t1953 + F::cast_from(0.2805166666666667_f64) * t2728 - F::cast_from(0.5610333333333334_f64) * t2730 - F::cast_from(0.6545388888888889_f64) * t2732 - F::cast_from(0.4630888888888889_f64) * t2061;
    let t3059 = t3058 * t371;
    let t3063 = F::cast_from(1.0_f64) / t987 / t363;
    (t3046, t3058, t3059, t3063)
}
