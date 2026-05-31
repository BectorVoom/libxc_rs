//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 849/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk849<F: Float>(t11409: F, t11411: F, t11413: F, t11415: F, t11557: F, t16046: F, t16048: F, t16051: F, t16052: F, t16057: F, t16062: F, t16067: F, t16071: F, t16075: F, t16080: F, t16084: F, t16088: F) -> F {
    let t16090 = -t11557 - F::cast_from(0.15829629629629629629e-1_f64) * t11409 + F::cast_from(0.39574074074074074073e-2_f64) * t11411 - F::cast_from(0.11872222222222222222e-1_f64) * t11413 + F::cast_from(0.5936111111111111111e-2_f64) * t11415 - F::cast_from(0.79148148148148148146e-2_f64) * t16046 + F::cast_from(0.79148148148148148146e-2_f64) * t16048 - t16051 - F::cast_from(0.13059444444444444444e0_f64) * t16052 - F::cast_from(0.19787037037037037037e-1_f64) * t16057 + F::cast_from(0.71233333333333333332e-1_f64) * t16062 + F::cast_from(0.47488888888888888888e-1_f64) * t16067 - F::cast_from(0.11872222222222222222e-1_f64) * t16071 - F::cast_from(0.10685e0_f64) * t16075 - F::cast_from(0.14246666666666666666e0_f64) * t16080 + F::cast_from(0.35616666666666666666e-1_f64) * t16084 + F::cast_from(0.35616666666666666666e-1_f64) * t16088;
    t16090
}
