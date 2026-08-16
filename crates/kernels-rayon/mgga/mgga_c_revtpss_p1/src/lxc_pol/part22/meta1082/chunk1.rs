//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3902/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3902(t13921: f64, t14193: f64, t22005: f64, t46416: f64, t46443: f64, t46448: f64, t46452: f64, t47976: f64, t47978: f64, t47980: f64, t47985: f64, t5767: f64, t74935: f64, t74943: f64, t74945: f64, t74949: f64, t820: f64) -> f64 {
    let t74954 = -0.21951497276451705328e-1_f64 * t47976 - 0.29268663035268940438e-1_f64 * t47978 - 0.29268663035268940438e-1_f64 * t47980 - 0.39512695097613069591e1_f64 * t14193 * t22005 * t46416 - 0.43902994552903410656e-1_f64 * t74935 + 0.58537326070537880875e-1_f64 * t47985 + 0.2601984143835408805e-1_f64 * t46443 + 0.13009920719177044025e-1_f64 * t46448 - 0.2601984143835408805e-1_f64 * t46452 + 0.21951497276451705328e-1_f64 * t74943 + 0.13009920719177044025e-2_f64 * t74945 + 0.78059524315062264149e-1_f64 * t74949 - 0.13170898365871023197e1_f64 * t820 * t5767 * t13921;
    t74954
}
