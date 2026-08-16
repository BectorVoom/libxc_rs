//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 971/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk971<F: Float>(t16050: F, t11409: F, t11411: F, t11413: F, t11415: F, t12791: F, t16046: F, t16048: F, t16052: F, t16057: F, t16062: F, t16067: F, t16071: F, t16075: F, t16080: F, t16084: F, t16088: F) -> F {
    let t17905 = F::cast_from(0.2283111111111111111e-1_f64) * t16050;
    let t17915 = -t12791 - F::cast_from(0.1522074074074074074e-1_f64) * t11409 + F::cast_from(0.38051851851851851851e-2_f64) * t11411 - F::cast_from(0.11415555555555555555e-1_f64) * t11413 + F::cast_from(0.57077777777777777777e-2_f64) * t11415 - F::cast_from(0.76103703703703703702e-2_f64) * t16046 + F::cast_from(0.76103703703703703701e-2_f64) * t16048 - t17905 - F::cast_from(0.1255711111111111111e0_f64) * t16052 - F::cast_from(0.19025925925925925925e-1_f64) * t16057 + F::cast_from(0.68493333333333333331e-1_f64) * t16062 + F::cast_from(0.45662222222222222221e-1_f64) * t16067 - F::cast_from(0.11415555555555555555e-1_f64) * t16071 - F::cast_from(0.10274e0_f64) * t16075 - F::cast_from(0.13698666666666666666e0_f64) * t16080 + F::cast_from(0.34246666666666666666e-1_f64) * t16084 + F::cast_from(0.34246666666666666666e-1_f64) * t16088;
    t17915
}
