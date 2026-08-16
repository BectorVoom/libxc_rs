//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1424/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1424(t27112: f64, t55917: f64, t1: f64, t1111: f64, t1121: f64, t1133: f64, t1506: f64, t15274: f64, t15327: f64, t15690: f64, t16241: f64, t17667: f64, t17724: f64, t17858: f64, t17922: f64, t3116: f64, t3119: f64, t322: f64, t35379: f64, t35559: f64, t4310: f64, t4363: f64, t4369: f64, t4374: f64, t438: f64, t450: f64, t46717: f64, t46792: f64, t5286: f64, t5302: f64, t54305: f64, t54317: f64, t54451: f64, t59434: f64, t59448: f64, t59452: f64, t59458: f64, t59462: f64, t59468: f64, t8966: f64) -> (f64, f64) {
    let t59474 = t27112 * t55917;
    let t59482 = -0.36629113921839320676e2_f64 * t8966 * t54451 * t15274 + 0.61174099372587555274e0_f64 * t15327 * t5302 - 0.75734008510040627576e0_f64 * t4363 * t17858 + 0.35500316489081544176e-1_f64 * t1121 * t450 * t59434 * t1 * t438 + 0.94667510637550784468e-1_f64 * t3116 * t4374 * t16241 * t1506 * t3119 - 0.17171677016866682182e0_f64 * t4369 * t17724 + 0.18110753103726578864e-2_f64 * t1133 * t59448 + 0.2951381987273961e-1_f64 * t1133 * t59452 + 0.21464596271083352727e-1_f64 * t54305 + 0.6104852320306553446e1_f64 * t46717 + 0.42074449172244793097e-1_f64 * t35379 + t1111 * t322 * t59458 / 288.0_f64 + 35.0_f64 / 972.0_f64 * t1111 * t322 * t59462 + 11.0_f64 / 54.0_f64 * t15690 * t5286 - 7.0_f64 / 54.0_f64 * t1111 * t322 * t59468 - 2.0_f64 / 9.0_f64 * t4310 * t17667 - t1111 * t322 * t59474 / 12.0_f64 + 0.4893927949807004422e0_f64 * t54317 - t46792 / 162.0_f64 - 0.39071054849961942054e3_f64 * t35559 * t17922;
    (t59474, t59482)
}
