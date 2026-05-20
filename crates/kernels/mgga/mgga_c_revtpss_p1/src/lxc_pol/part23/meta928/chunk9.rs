//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3030/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3030<F: Float>(t1024: F, t1082: F, t11788: F, t12122: F, t12167: F, t12168: F, t16381: F, t16502: F, t19380: F, t19526: F, t19569: F, t19573: F, t19576: F, t19612: F, t20136: F, t23964: F, t24138: F, t24144: F, t3204: F, t3291: F, t3304: F, t43432: F, t4980: F, t4984: F, t5004: F, t6235: F, t6379: F, t78826: F, t79275: F, t80341: F) -> F {
    let t80724 = -F::cast_from(0.39512695097613069591e1_f64) * t43432 * t24138 - F::cast_from(0.39512695097613069591e1_f64) * t12122 * t79275 * t3304 - F::cast_from(0.39512695097613069591e1_f64) * t16502 * t20136 + F::cast_from(0.39512695097613069591e1_f64) * t11788 * t24144 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t3291 * t23964 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t1082 * t78826 + F::cast_from(0.39512695097613069591e1_f64) * t12167 * t80341 * t12168 + F::cast_from(0.39512695097613069591e1_f64) * t16381 * t6379 - F::cast_from(0.19756347548806534796e1_f64) * t1024 * t5004 * t19380 - F::cast_from(0.19756347548806534796e1_f64) * t16502 * t19612 + F::cast_from(0.39512695097613069592e1_f64) * t19526 * t19573 - F::cast_from(0.19756347548806534796e1_f64) * t19569 * t19576 + F::cast_from(0.39512695097613069592e1_f64) * t6235 * t4980 * t4984;
    t80724
}
