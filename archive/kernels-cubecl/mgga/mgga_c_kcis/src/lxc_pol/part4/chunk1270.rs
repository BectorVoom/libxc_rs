//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1270/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1270<F: Float>(t11475: F, t16046: F, t16052: F, t16057: F, t16067: F, t16071: F, t16075: F, t16080: F, t16084: F, t16127: F, t16129: F, t16132: F, t16135: F, t16137: F, t16142: F, t16145: F, t16146: F, t16160: F, t16163: F, t16165: F, t16168: F, t16221: F) -> F {
    let t16223 = -F::cast_from(0.71202444444444444443e0_f64) * t16127 - F::cast_from(0.91285185185185185185e-1_f64) * t16129 - F::cast_from(0.76790625e-1_f64) * t16132 - F::cast_from(0.1898925e1_f64) * t16135 - F::cast_from(0.9494625e0_f64) * t16137 - F::cast_from(0.21924222222222222222e1_f64) * t16052 - F::cast_from(0.13287407407407407408e0_f64) * t16046 - F::cast_from(0.65725333333333333332e0_f64) * t16142 - t16145 + F::cast_from(0.36514074074074074074e-1_f64) * t16146 + t16160 + F::cast_from(0.3071625e0_f64) * t16163 + F::cast_from(0.15358125e0_f64) * t16165 + F::cast_from(0.142419375e1_f64) * t16168 - F::cast_from(0.33218518518518518518e0_f64) * t16057 + F::cast_from(0.79724444444444444445e0_f64) * t16067 - F::cast_from(0.19931111111111111111e0_f64) * t16071 - F::cast_from(0.17938e1_f64) * t16075 - F::cast_from(0.23917333333333333334e1_f64) * t16080 + F::cast_from(0.59793333333333333334e0_f64) * t16084 - F::cast_from(0.10954222222222222222e0_f64) * t11475 + t16221;
    t16223
}
