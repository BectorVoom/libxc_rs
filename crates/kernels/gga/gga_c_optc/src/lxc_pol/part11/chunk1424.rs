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
    let t59482 = -F::new(0.36629113921839320676e2) * t8966 * t54451 * t15274 + F::new(0.61174099372587555274e0) * t15327 * t5302 - F::new(0.75734008510040627576e0) * t4363 * t17858 + F::new(0.35500316489081544176e-1) * t1121 * t450 * t59434 * t1 * t438 + F::new(0.94667510637550784468e-1) * t3116 * t4374 * t16241 * t1506 * t3119 - F::new(0.17171677016866682182e0) * t4369 * t17724 + F::new(0.18110753103726578864e-2) * t1133 * t59448 + F::new(0.2951381987273961e-1) * t1133 * t59452 + F::new(0.21464596271083352727e-1) * t54305 + F::new(0.6104852320306553446e1) * t46717 + F::new(0.42074449172244793097e-1) * t35379 + t1111 * t322 * t59458 / F::new(288.0) + F::new(35.0) / F::new(972.0) * t1111 * t322 * t59462 + F::new(11.0) / F::new(54.0) * t15690 * t5286 - F::new(7.0) / F::new(54.0) * t1111 * t322 * t59468 - F::new(2.0) / F::new(9.0) * t4310 * t17667 - t1111 * t322 * t59474 / F::new(12.0) + F::new(0.4893927949807004422e0) * t54317 - t46792 / F::new(162.0) - F::new(0.39071054849961942054e3) * t35559 * t17922;
    (t59474, t59482)
}
