//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1085/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1085(t1882: f64, t9831: f64, t10123: f64, t3281: f64, t768: f64, t2559: f64, t8232: f64, t2563: f64, t731: f64, t10092: f64, t1901: f64, t1934: f64, t2405: f64, t2526: f64, t2606: f64, t2607: f64, t3885: f64, t3891: f64, t42399: f64, t766: f64, t8608: f64, t9787: f64, t9854: f64) -> f64 {
    let t42606 = t1882 * t9831;
    let t42608 = t1882 * t10123;
    let t42610 = t3281 * t768;
    let t42612 = t8232 * t2559;
    let t42614 = t8232 * t2563;
    let t42616 = t3281 * t731;
    let t42639 = 8.0_f64 / 9.0_f64 * t42606 + 4.0_f64 / 9.0_f64 * t42608 + 112.0_f64 / 81.0_f64 * t42610 - 16.0_f64 / 9.0_f64 * t42612 - 8.0_f64 / 9.0_f64 * t42614 + 112.0_f64 / 81.0_f64 * t42616 + 4.0_f64 / 9.0_f64 * t1901 * t3891 * t10092 * t2405 + 4.0_f64 / 9.0_f64 * t1901 * t2606 * t2607 * t8608 * t766 + 2.0_f64 / 3.0_f64 * t1901 * t2606 * t2607 * t1934 * t2526 + 8.0_f64 / 3.0_f64 * t1901 * t9787 * t9854 + 8.0_f64 / 9.0_f64 * t1901 * t2606 * t3885 * t42399;
    t42639
}
