//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3902/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3902<F: Float>(t13921: F, t14193: F, t22005: F, t46416: F, t46443: F, t46448: F, t46452: F, t47976: F, t47978: F, t47980: F, t47985: F, t5767: F, t74935: F, t74943: F, t74945: F, t74949: F, t820: F) -> F {
    let t74954 = -F::cast_from(0.21951497276451705328e-1_f64) * t47976 - F::cast_from(0.29268663035268940438e-1_f64) * t47978 - F::cast_from(0.29268663035268940438e-1_f64) * t47980 - F::cast_from(0.39512695097613069591e1_f64) * t14193 * t22005 * t46416 - F::cast_from(0.43902994552903410656e-1_f64) * t74935 + F::cast_from(0.58537326070537880875e-1_f64) * t47985 + F::cast_from(0.2601984143835408805e-1_f64) * t46443 + F::cast_from(0.13009920719177044025e-1_f64) * t46448 - F::cast_from(0.2601984143835408805e-1_f64) * t46452 + F::cast_from(0.21951497276451705328e-1_f64) * t74943 + F::cast_from(0.13009920719177044025e-2_f64) * t74945 + F::cast_from(0.78059524315062264149e-1_f64) * t74949 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t5767 * t13921;
    t74954
}
