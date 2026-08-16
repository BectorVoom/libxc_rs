//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1146/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1146(t1131: f64, t21181: f64, t1091: f64, t1175: f64, t14081: f64, t14159: f64, t1901: f64, t21369: f64, t21764: f64, t21772: f64, t2599: f64, t3885: f64, t3891: f64, t42409: f64, t42859: f64, t446: f64, t4917: f64, t5053: f64, t68074: f64, t724: f64, t81170: f64, t81183: f64, t81207: f64, t81209: f64, t88098: f64) -> (f64, f64) {
    let t89222 = t21181 * t1131;
    let t89252 = 40.0_f64 / 81.0_f64 * t1901 * t42409 * t14081 * t89222 + 40.0_f64 / 243.0_f64 * t81170 + 8.0_f64 / 27.0_f64 * t81183 - 4.0_f64 / 9.0_f64 * t446 * t724 * t21772 * t1091 - 4.0_f64 / 9.0_f64 * t446 * t724 * t1175 * t21369 - 8.0_f64 / 9.0_f64 * t68074 + 8.0_f64 / 3.0_f64 * t81207 + 8.0_f64 / 27.0_f64 * t81209 - 16.0_f64 / 9.0_f64 * t1901 * t3891 * t42859 * t88098 - 8.0_f64 / 3.0_f64 * t1901 * t14159 * t21764 - 4.0_f64 / 3.0_f64 * t1901 * t2599 * t3885 * t4917 * t5053;
    (t89222, t89252)
}
