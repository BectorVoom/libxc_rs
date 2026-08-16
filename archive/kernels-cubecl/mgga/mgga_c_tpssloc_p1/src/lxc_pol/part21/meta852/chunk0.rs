//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3081/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3081<F: Float>(t1113: F, t136: F, t63294: F, t63298: F, t63302: F, t2403: F, t6017: F, t11219: F, t63415: F, t43748: F, t63332: F, t63334: F, t63336: F, t63886: F, t63888: F, t63891: F, t63893: F, t63896: F, t63899: F) -> (F, F, F, F, F, F) {
    let t63903 = t136 * t1113 * t63294;
    let t63906 = t136 * t1113 * t63298;
    let t63909 = t136 * t1113 * t63302;
    let t63911 = t2403 * t6017;
    let t63914 = t136 * t11219 * t63415;
    let t63916 = -F::cast_from(0.88582716049382716049e-1_f64) * t63332 + F::cast_from(0.13287407407407407408e0_f64) * t63334 - F::cast_from(0.19931111111111111111e0_f64) * t63336 - F::cast_from(0.10954222222222222222e0_f64) * t63886 - F::cast_from(0.30428395061728395062e-1_f64) * t63888 - F::cast_from(0.54771111111111111112e-1_f64) * t63891 + F::cast_from(0.18257037037037037037e0_f64) * t63893 + F::cast_from(0.32862666666666666666e0_f64) * t63896 + F::cast_from(0.1460562962962962963e0_f64) * t63899 - F::cast_from(0.88582716049382716053e-1_f64) * t43748 + F::cast_from(0.32862666666666666666e0_f64) * t63903 + F::cast_from(0.16431333333333333333e0_f64) * t63906 + F::cast_from(0.49293999999999999999e0_f64) * t63909 + F::cast_from(0.91285185185185185185e-1_f64) * t63911 + F::cast_from(0.36514074074074074075e-1_f64) * t63914;
    (t63903, t63906, t63909, t63911, t63914, t63916)
}
