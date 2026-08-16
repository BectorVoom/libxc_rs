//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2693/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2693(t3899: f64, t5775: f64, t689: f64, t14100: f64, t9686: f64, t13729: f64, t2782: f64, t4131: f64, t556: f64, t47506: f64, t5722: f64, t10171: f64, t1424: f64, t14268: f64, t1444: f64, t4076: f64, t47593: f64, t47595: f64, t47601: f64, t47606: f64, t47608: f64, t47612: f64, t47616: f64, t47618: f64, t47620: f64, t5715: f64, t5728: f64, t9659: f64) -> f64 {
    let t49508 = t689 * t3899 * t5775;
    let t49512 = t14100 * t9686;
    let t49513 = 0.39029762157531132076e-1_f64 * t49512;
    let t49522 = t2782 * t556 * t13729 * t4131;
    let t49528 = t47506 * t5722;
    let t49534 = 0.32927245914677557992e-1_f64 * t49508 + 0.16463622957338778996e-1_f64 * t47593 - 0.21951497276451705329e-1_f64 * t47595 + t47601 + t49513 + 0.39512695097613069591e1_f64 * t10171 * t5728 + 0.39512695097613069591e1_f64 * t1424 * t4076 * t14268 * t1444 - 0.32927245914677557992e-1_f64 * t49522 - 0.58544643236296698114e-1_f64 * t47606 + 0.43902994552903410657e-1_f64 * t47608 - 0.39512695097613069591e1_f64 * t5715 * t9659 - 0.29272321618148349057e-1_f64 * t49528 + 0.58544643236296698114e-1_f64 * t47612 - 0.19514881078765566037e-2_f64 * t47616 + 0.7805952431506226415e-2_f64 * t47618 + 0.21951497276451705329e-1_f64 * t47620;
    t49534
}
