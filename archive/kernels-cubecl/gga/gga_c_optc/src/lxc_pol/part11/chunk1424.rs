//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1424/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1424<F: Float>(t27112: F, t55917: F, t1: F, t1111: F, t1121: F, t1133: F, t1506: F, t15274: F, t15327: F, t15690: F, t16241: F, t17667: F, t17724: F, t17858: F, t17922: F, t3116: F, t3119: F, t322: F, t35379: F, t35559: F, t4310: F, t4363: F, t4369: F, t4374: F, t438: F, t450: F, t46717: F, t46792: F, t5286: F, t5302: F, t54305: F, t54317: F, t54451: F, t59434: F, t59448: F, t59452: F, t59458: F, t59462: F, t59468: F, t8966: F) -> (F, F) {
    let t59474 = t27112 * t55917;
    let t59482 = -F::cast_from(0.36629113921839320676e2_f64) * t8966 * t54451 * t15274 + F::cast_from(0.61174099372587555274e0_f64) * t15327 * t5302 - F::cast_from(0.75734008510040627576e0_f64) * t4363 * t17858 + F::cast_from(0.35500316489081544176e-1_f64) * t1121 * t450 * t59434 * t1 * t438 + F::cast_from(0.94667510637550784468e-1_f64) * t3116 * t4374 * t16241 * t1506 * t3119 - F::cast_from(0.17171677016866682182e0_f64) * t4369 * t17724 + F::cast_from(0.18110753103726578864e-2_f64) * t1133 * t59448 + F::cast_from(0.2951381987273961e-1_f64) * t1133 * t59452 + F::cast_from(0.21464596271083352727e-1_f64) * t54305 + F::cast_from(0.6104852320306553446e1_f64) * t46717 + F::cast_from(0.42074449172244793097e-1_f64) * t35379 + t1111 * t322 * t59458 / F::cast_from(288.0_f64) + F::cast_from(35.0_f64) / F::cast_from(972.0_f64) * t1111 * t322 * t59462 + F::cast_from(11.0_f64) / F::cast_from(54.0_f64) * t15690 * t5286 - F::cast_from(7.0_f64) / F::cast_from(54.0_f64) * t1111 * t322 * t59468 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4310 * t17667 - t1111 * t322 * t59474 / F::cast_from(12.0_f64) + F::cast_from(0.4893927949807004422e0_f64) * t54317 - t46792 / F::cast_from(162.0_f64) - F::cast_from(0.39071054849961942054e3_f64) * t35559 * t17922;
    (t59474, t59482)
}
