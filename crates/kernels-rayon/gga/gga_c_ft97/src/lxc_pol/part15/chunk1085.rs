//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1085/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1085(t4417: f64, t4724: f64, t1060: f64, t17164: f64, t1901: f64, t20027: f64, t20731: f64, t20749: f64, t20754: f64, t2205: f64, t2210: f64, t3434: f64, t3439: f64, t3440: f64, t40911: f64, t446: f64, t49634: f64, t49661: f64, t63530: f64, t63536: f64, t63613: f64, t76777: f64, t85320: f64, t925: f64) -> (f64, f64) {
    let t87462 = t4417 * t4724;
    let t87517 = 8.0_f64 / 9.0_f64 * t1901 * t2210 * t3434 * t85320 - 8.0_f64 / 27.0_f64 * t1901 * t3439 * t3440 * t85320 + 8.0_f64 / 9.0_f64 * t1901 * t49634 * t20749 + 8.0_f64 / 9.0_f64 * t1901 * t17164 * t20754 - 16.0_f64 / 27.0_f64 * t63530 - 8.0_f64 / 9.0_f64 * t63536 + 16.0_f64 / 9.0_f64 * t446 * t2205 * t1060 * t20027 + 4.0_f64 / 9.0_f64 * t76777 + 112.0_f64 / 81.0_f64 * t49661 + 8.0_f64 / 3.0_f64 * t1901 * t40911 * t20731 * t925 + 16.0_f64 / 27.0_f64 * t63613;
    (t87462, t87517)
}
