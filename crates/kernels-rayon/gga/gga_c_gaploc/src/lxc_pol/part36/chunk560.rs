//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 560/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk560(t2530: f64, t2667: f64, t1445: f64, t2033: f64, t2087: f64, t2639: f64, t3277: f64, t6111: f64, t9832: f64, t9836: f64, t9838: f64, t9839: f64, t9842: f64, t9846: f64, t9849: f64, t9853: f64, t9854: f64, t9857: f64, t9858: f64) -> f64 {
    let t9863 = t2667 * t2530;
    let t9864 = t1445 * t9863;
    let t9867 = t9832 - t9836 + t9838 - 0.79445533226334281487e-1_f64 * t6111 * t9839 + 0.39722766613167140743e-1_f64 * t2033 * t9842 - t9846 - t9849 + t9853 - 0.10725146985555128001e1_f64 * t9854 * t2639 + 0.42900587942220512003e1_f64 * t9857 * t9858 - 0.25025342966295298669e1_f64 * t3277 * t2639 - 0.13803453343411469884e2_f64 * t2087 * t9864;
    t9867
}
