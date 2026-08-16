//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1268/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1268(t1094: f64, t1102: f64, t26164: f64, t8749: f64, t3067: f64, t8550: f64, t3054: f64, t3074: f64, t8555: f64, t3061: f64, t8697: f64, t8740: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26168 = 0.1403573615389248977e2_f64 * t1102 * t8749 * t26164 * t1094;
    let t26170 = 0.1403573615389248977e2_f64 * t3067 * t8550;
    let t26173 = 0.21053604230838734656e2_f64 * t1102 * t3074 * t3054;
    let t26175 = 0.14035736153892489771e2_f64 * t3067 * t8555;
    let t26179 = 0.6233672123775310788e3_f64 * t1102 * t8697 * t26164 * t3061;
    let t26181 = 0.23392893589820816284e1_f64 * t3067 * t8740;
    (t26168, t26170, t26173, t26175, t26179, t26181)
}
