//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2693/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2693<F: Float>(t3899: F, t5775: F, t689: F, t14100: F, t9686: F, t13729: F, t2782: F, t4131: F, t556: F, t47506: F, t5722: F, t10171: F, t1424: F, t14268: F, t1444: F, t4076: F, t47593: F, t47595: F, t47601: F, t47606: F, t47608: F, t47612: F, t47616: F, t47618: F, t47620: F, t5715: F, t5728: F, t9659: F) -> F {
    let t49508 = t689 * t3899 * t5775;
    let t49512 = t14100 * t9686;
    let t49513 = F::cast_from(0.39029762157531132076e-1_f64) * t49512;
    let t49522 = t2782 * t556 * t13729 * t4131;
    let t49528 = t47506 * t5722;
    let t49534 = F::cast_from(0.32927245914677557992e-1_f64) * t49508 + F::cast_from(0.16463622957338778996e-1_f64) * t47593 - F::cast_from(0.21951497276451705329e-1_f64) * t47595 + t47601 + t49513 + F::cast_from(0.39512695097613069591e1_f64) * t10171 * t5728 + F::cast_from(0.39512695097613069591e1_f64) * t1424 * t4076 * t14268 * t1444 - F::cast_from(0.32927245914677557992e-1_f64) * t49522 - F::cast_from(0.58544643236296698114e-1_f64) * t47606 + F::cast_from(0.43902994552903410657e-1_f64) * t47608 - F::cast_from(0.39512695097613069591e1_f64) * t5715 * t9659 - F::cast_from(0.29272321618148349057e-1_f64) * t49528 + F::cast_from(0.58544643236296698114e-1_f64) * t47612 - F::cast_from(0.19514881078765566037e-2_f64) * t47616 + F::cast_from(0.7805952431506226415e-2_f64) * t47618 + F::cast_from(0.21951497276451705329e-1_f64) * t47620;
    t49534
}
