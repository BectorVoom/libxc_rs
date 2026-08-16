//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2545/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2545<F: Float>(t11352: F, t4819: F, t11303: F, t11306: F, t11344: F, t11350: F, t1136: F, t11415: F, t11420: F, t11430: F, t15117: F, t15136: F, t15156: F, t15159: F, t15164: F, t15165: F, t15168: F, t15171: F, t15172: F, t1682: F, t1683: F, t3332: F, t3333: F, t3351: F, t3357: F, t3359: F, t44172: F, t44177: F, t44179: F, t44214: F, t44361: F, t4820: F, t4823: F) -> F {
    let t51521 = t4819 * t11352;
    let t51538 = -F::cast_from(6.0_f64) * t3332 * t4820 * t3351 - F::cast_from(0.57895126195293126242e3_f64) * t11420 * t15164 * t3333 - F::cast_from(6.0_f64) * t11303 * t15156 - F::cast_from(2.0_f64) * t3332 * t1683 * t11344 - F::cast_from(0.57895126195293126242e3_f64) * t44214 * t15159 - F::cast_from(0.24828486201251232145e5_f64) * t44361 * t15171 * t11306 + F::cast_from(0.19298375398431042081e3_f64) * t11415 * t15165 + F::cast_from(0.96491876992155210402e2_f64) * t3357 * t15117 * t3359 * t1136 + F::cast_from(0.96491876992155210402e2_f64) * t3357 * t15164 * t3351 + F::cast_from(0.6207121550312808036e4_f64) * t11350 * t51521 * t3333 + F::cast_from(0.96491876992155210402e2_f64) * t11415 * t15168 + F::cast_from(0.32163958997385070134e2_f64) * t3357 * t4823 * t11344 + F::cast_from(0.6207121550312808036e4_f64) * t44172 * t15172 + F::cast_from(0.19964560303604640732e6_f64) * t44177 * t1682 * t44179 * t11306 - F::cast_from(0.35089341735807877242e1_f64) * t15136 * t11430;
    t51538
}
