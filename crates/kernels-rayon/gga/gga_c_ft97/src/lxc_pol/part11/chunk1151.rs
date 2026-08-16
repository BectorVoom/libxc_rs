//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1151/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1151(t2349: f64, t2844: f64, t875: f64, t9571: f64, t2803: f64, t8232: f64, t10488: f64, t8392: f64, t2739: f64, t10479: f64, t10485: f64, t15182: f64, t1901: f64, t1934: f64, t2857: f64, t2881: f64, t296: f64, t319: f64, t4140: f64, t41726: f64, t43948: f64, t44190: f64, t44195: f64, t44202: f64, t44204: f64, t446: f64) -> (f64, f64, f64, f64) {
    let t44205 = t2349 * t2844;
    let t44210 = t9571 * t875;
    let t44215 = t8232 * t2803;
    let t44217 = t8392 * t10488;
    let t44219 = t2349 * t2739;
    let t44224 = -2.0_f64 / 9.0_f64 * t446 * t2857 * t319 * t41726 - 16.0_f64 / 27.0_f64 * t44190 - 4.0_f64 / 3.0_f64 * t446 * t296 * t43948 + 4.0_f64 / 3.0_f64 * t44195 - 4.0_f64 / 3.0_f64 * t1901 * t2881 * t15182 * t1934 * t2844 + 8.0_f64 / 9.0_f64 * t44202 + 8.0_f64 / 3.0_f64 * t1901 * t2881 * t44204 * t44205 + 8.0_f64 / 3.0_f64 * t1901 * t2881 * t10485 * t44210 - 8.0_f64 / 9.0_f64 * t44215 - 8.0_f64 / 27.0_f64 * t44217 + 4.0_f64 / 9.0_f64 * t1901 * t10479 * t4140 * t44219;
    (t44205, t44210, t44219, t44224)
}
