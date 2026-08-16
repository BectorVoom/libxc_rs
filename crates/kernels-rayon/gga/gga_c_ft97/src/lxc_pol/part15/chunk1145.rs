//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1145/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1145(t20489: f64, t992: f64, t10007: f64, t1131: f64, t1168: f64, t13927: f64, t1901: f64, t21494: f64, t2599: f64, t2600: f64, t2606: f64, t2607: f64, t3885: f64, t3891: f64, t3892: f64, t446: f64, t4973: f64, t5073: f64, t51972: f64, t67961: f64, t68001: f64, t729: f64, t81162: f64, t81164: f64) -> (f64, f64) {
    let t89212 = t20489 * t992;
    let t89221 = -4.0_f64 / 3.0_f64 * t1901 * t10007 * t4973 * t5073 + 16.0_f64 / 9.0_f64 * t67961 - 8.0_f64 / 27.0_f64 * t68001 - 8.0_f64 * t446 * t729 * t13927 * t21494 + 112.0_f64 / 81.0_f64 * t51972 - 16.0_f64 / 27.0_f64 * t81162 + 4.0_f64 / 9.0_f64 * t81164 + 4.0_f64 / 9.0_f64 * t1901 * t2599 * t2600 * t20489 * t1131 + 4.0_f64 / 9.0_f64 * t1901 * t2606 * t2607 * t20489 * t1168 + 8.0_f64 / 9.0_f64 * t1901 * t2606 * t3885 * t89212 - 8.0_f64 / 27.0_f64 * t1901 * t3891 * t3892 * t89212;
    (t89212, t89221)
}
