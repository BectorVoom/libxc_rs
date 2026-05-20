//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2200/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2200<F: Float>(t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16727: F, t16748: F, t16869: F, t16871: F, t16873: F, t16876: F, t1145: F, t16742: F) -> (F, F) {
    let t16883 = -t16869 + F::cast_from(0.82156666666666666667e-1_f64) * t16871 - t16873 + F::cast_from(0.29896666666666666667e0_f64) * t16748 + F::cast_from(0.13287407407407407408e0_f64) * t16706 + F::cast_from(0.91285185185185185185e-1_f64) * t16876 + F::cast_from(0.66437037037037037038e-1_f64) * t12299 + F::cast_from(0.26574814814814814816e0_f64) * t12297 - F::cast_from(0.19931111111111111111e0_f64) * t12301 - F::cast_from(0.99655555555555555557e-1_f64) * t12303 - F::cast_from(0.39862222222222222222e0_f64) * t16727;
    let t16886 = t1145 * t16742;
    (t16883, t16886)
}
