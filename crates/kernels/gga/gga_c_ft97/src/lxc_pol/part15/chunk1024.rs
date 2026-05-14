//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1024/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1024<F: Float>(t43350: F, t446: F, t89881: F, t4969: F, t5299: F, t2665: F, t1212: F, t21355: F, t10409: F, t1193: F, t21373: F, t4977: F, t5252: F, t10327: F, t10339: F, t10355: F, t1095: F, t19168: F, t2014: F, t22110: F, t231: F, t2394: F, t2710: F, t274: F, t39926: F, t43691: F, t43731: F, t4635: F, t4917: F, t5248: F, t683: F, t70278: F, t70290: F, t70326: F, t70354: F, t801: F, t82988: F, t83084: F, t83086: F, t83088: F, t83103: F, t88503: F, t8948: F, t8963: F, t9609: F) -> (F, F, F, F, F, F) {
    let t89883 = t446 * t43350 * t89881;
    let t89885 = t4969 * t5299;
    let t89887 = t446 * t2665 * t89885;
    let t89889 = t21355 * t1212;
    let t89891 = t446 * t10409 * t89889;
    let t89893 = t1193 * t21373;
    let t89896 = t5252 * t4977;
    let t89941 = 0.1279131955121726244e0 * t2710 * t89893 - 0.15095674251318553494e0 * t9609 * t89896 + 0.55909904634513161088e-1 * t2394 * t89893 - 0.5498505610292168117e-2 * t10355 * t89896 - 0.30699166922921429856e0 * t10339 * t89896 + t70354 - 0.90429780618718677442e-4 * t8948 * t683 * t88503 * t801 * t274 - 0.44273842265453930305e-2 * t83084 - 0.59031789687271907074e-3 * t83086 + 0.48229216329983294636e-3 * t83088 - 0.8854768453090786061e-3 * t8963 * t19168 * t10327 * t4635 - 0.11806357937454381415e-2 * t8963 * t70326 * t43731 * t4917 + 0.43406294696984965172e-2 * t8963 * t70290 * t70278 * t274 + 0.17709536906181572122e-2 * t8963 * t19168 * t43691 * t4917 + 0.22136921132726965153e-3 * t39926 * t82988 * t22110 + t83103 - 0.10625722143708943273e-1 * t2014 * t231 * t21373 * t1095 * t274 + 0.19923229019454268637e-2 * t8948 * t683 * t5248 * t4977;
    (t89883, t89885, t89887, t89889, t89891, t89941)
}
