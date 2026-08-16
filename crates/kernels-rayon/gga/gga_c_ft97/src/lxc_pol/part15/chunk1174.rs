//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1174/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1174(t4977: f64, t5252: f64, t10327: f64, t10339: f64, t10355: f64, t1095: f64, t19168: f64, t2014: f64, t21373: f64, t22110: f64, t231: f64, t2394: f64, t2710: f64, t274: f64, t39926: f64, t43691: f64, t43731: f64, t4635: f64, t4917: f64, t5248: f64, t683: f64, t70278: f64, t70290: f64, t70326: f64, t70354: f64, t801: f64, t82988: f64, t83084: f64, t83086: f64, t83088: f64, t83103: f64, t88503: f64, t8948: f64, t8963: f64, t89893: f64, t9609: f64) -> f64 {
    let t89896 = t5252 * t4977;
    let t89941 = 0.1279131955121726244e0_f64 * t2710 * t89893 - 0.15095674251318553494e0_f64 * t9609 * t89896 + 0.55909904634513161088e-1_f64 * t2394 * t89893 - 0.5498505610292168117e-2_f64 * t10355 * t89896 - 0.30699166922921429856e0_f64 * t10339 * t89896 + t70354 - 0.90429780618718677442e-4_f64 * t8948 * t683 * t88503 * t801 * t274 - 0.44273842265453930305e-2_f64 * t83084 - 0.59031789687271907074e-3_f64 * t83086 + 0.48229216329983294636e-3_f64 * t83088 - 0.8854768453090786061e-3_f64 * t8963 * t19168 * t10327 * t4635 - 0.11806357937454381415e-2_f64 * t8963 * t70326 * t43731 * t4917 + 0.43406294696984965172e-2_f64 * t8963 * t70290 * t70278 * t274 + 0.17709536906181572122e-2_f64 * t8963 * t19168 * t43691 * t4917 + 0.22136921132726965153e-3_f64 * t39926 * t82988 * t22110 + t83103 - 0.10625722143708943273e-1_f64 * t2014 * t231 * t21373 * t1095 * t274 + 0.19923229019454268637e-2_f64 * t8948 * t683 * t5248 * t4977;
    t89941
}
