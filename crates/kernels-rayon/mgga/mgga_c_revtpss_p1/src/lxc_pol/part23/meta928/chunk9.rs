//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3030/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3030(t1024: f64, t1082: f64, t11788: f64, t12122: f64, t12167: f64, t12168: f64, t16381: f64, t16502: f64, t19380: f64, t19526: f64, t19569: f64, t19573: f64, t19576: f64, t19612: f64, t20136: f64, t23964: f64, t24138: f64, t24144: f64, t3204: f64, t3291: f64, t3304: f64, t43432: f64, t4980: f64, t4984: f64, t5004: f64, t6235: f64, t6379: f64, t78826: f64, t79275: f64, t80341: f64) -> f64 {
    let t80724 = -0.39512695097613069591e1_f64 * t43432 * t24138 - 0.39512695097613069591e1_f64 * t12122 * t79275 * t3304 - 0.39512695097613069591e1_f64 * t16502 * t20136 + 0.39512695097613069591e1_f64 * t11788 * t24144 + 0.39512695097613069591e1_f64 * t3204 * t3291 * t23964 + 0.39512695097613069591e1_f64 * t3204 * t1082 * t78826 + 0.39512695097613069591e1_f64 * t12167 * t80341 * t12168 + 0.39512695097613069591e1_f64 * t16381 * t6379 - 0.19756347548806534796e1_f64 * t1024 * t5004 * t19380 - 0.19756347548806534796e1_f64 * t16502 * t19612 + 0.39512695097613069592e1_f64 * t19526 * t19573 - 0.19756347548806534796e1_f64 * t19569 * t19576 + 0.39512695097613069592e1_f64 * t6235 * t4980 * t4984;
    t80724
}
