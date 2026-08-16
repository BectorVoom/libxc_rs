//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2545/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2545(t11352: f64, t4819: f64, t11303: f64, t11306: f64, t11344: f64, t11350: f64, t1136: f64, t11415: f64, t11420: f64, t11430: f64, t15117: f64, t15136: f64, t15156: f64, t15159: f64, t15164: f64, t15165: f64, t15168: f64, t15171: f64, t15172: f64, t1682: f64, t1683: f64, t3332: f64, t3333: f64, t3351: f64, t3357: f64, t3359: f64, t44172: f64, t44177: f64, t44179: f64, t44214: f64, t44361: f64, t4820: f64, t4823: f64) -> f64 {
    let t51521 = t4819 * t11352;
    let t51538 = -6.0_f64 * t3332 * t4820 * t3351 - 0.57895126195293126242e3_f64 * t11420 * t15164 * t3333 - 6.0_f64 * t11303 * t15156 - 2.0_f64 * t3332 * t1683 * t11344 - 0.57895126195293126242e3_f64 * t44214 * t15159 - 0.24828486201251232145e5_f64 * t44361 * t15171 * t11306 + 0.19298375398431042081e3_f64 * t11415 * t15165 + 0.96491876992155210402e2_f64 * t3357 * t15117 * t3359 * t1136 + 0.96491876992155210402e2_f64 * t3357 * t15164 * t3351 + 0.6207121550312808036e4_f64 * t11350 * t51521 * t3333 + 0.96491876992155210402e2_f64 * t11415 * t15168 + 0.32163958997385070134e2_f64 * t3357 * t4823 * t11344 + 0.6207121550312808036e4_f64 * t44172 * t15172 + 0.19964560303604640732e6_f64 * t44177 * t1682 * t44179 * t11306 - 0.35089341735807877242e1_f64 * t15136 * t11430;
    t51538
}
