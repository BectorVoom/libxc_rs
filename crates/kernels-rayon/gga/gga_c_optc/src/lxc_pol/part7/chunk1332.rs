//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1332/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1332(t26508: f64, t26523: f64, t26539: f64, t26554: f64, t1085: f64, t1094: f64, t1102: f64, t2968: f64, t2936: f64, t1075: f64, t26152: f64, t26156: f64, t26160: f64, t26163: f64, t26164: f64, t26192: f64, t26200: f64, t26203: f64, t26206: f64, t26209: f64, t26212: f64, t26229: f64, t2974: f64, t2976: f64, t3035: f64, t3059: f64, t3061: f64, t8757: f64, t8762: f64, t8765: f64, t8781: f64, t8786: f64, t8809: f64) -> (f64, f64, f64, f64, f64) {
    let t26556 = t26508 + t26523 + t26539 + t26554;
    let t26560 = 0.58482233974552040708e0_f64 * t1102 * t1085 * t26556 * t1094;
    let t26561 = t2968 * t2968;
    let t26578 = t2936 * t2936;
    let t26582 = 0.96494049533612093922e2_f64 * t2974 * t26561 * t2976 + 0.14035736153892489771e2_f64 * t8762 * t8757 - 0.1403573615389248977e2_f64 * t8765 * t26164 * t1094 - 0.35089340384731224426e1_f64 * t3035 * t26229 * t1094 + 0.51947267698127589897e2_f64 * t3059 * t26229 * t3061 + 24.0_f64 * t8781 * t8809 - 24.0_f64 * t8786 * t26578 * t1075 - t26152 + t26156 + t26160 - t26163 - t26192 - t26200 + t26203 + t26206 - t26209 - t26212;
    (t26556, t26560, t26561, t26578, t26582)
}
