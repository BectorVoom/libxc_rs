//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1598/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1598(t1480: f64, t1483: f64, t21732: f64, t21754: f64, t22671: f64, t22700: f64, t22709: f64, t22712: f64, t22715: f64, t2275: f64, t2282: f64, t4201: f64, t4210: f64, t44: f64, t46065: f64, t46074: f64, t46090: f64, t48: f64, t56: f64, t5825: f64, t5843: f64, t5848: f64, t5851: f64, t60: f64, t61: f64, t87107: f64, t87126: f64, t87132: f64, t87145: f64) -> f64 {
    let t87155 = -5.0_f64 / 18.0_f64 * t44 * t21732 * t5825 + 5.0_f64 / 6.0_f64 * t44 * t2275 * t87107 + 10.0_f64 / 9.0_f64 * t44 * t4201 * t22671 - 80.0_f64 / 9.0_f64 * t1480 * t22712 + 5.0_f64 / 18.0_f64 * t56 * t21754 * t5825 + 5.0_f64 / 6.0_f64 * t56 * t2282 * t87107 + 10.0_f64 / 9.0_f64 * t56 * t4210 * t22671 + 5.0_f64 / 6.0_f64 * t44 * t48 * t87126 + 20944.0_f64 / 81.0_f64 * t87132 * t61 + 12320.0_f64 / 81.0_f64 * t22700 * t1483 - 440.0_f64 / 9.0_f64 * t5843 * t5851 + 440.0_f64 / 27.0_f64 * t5843 * t5848 - 40.0_f64 / 81.0_f64 * t1480 * t22709 + 80.0_f64 / 9.0_f64 * t1480 * t22715 + 5.0_f64 / 162.0_f64 * t56 * t46074 * t87145 - 5.0_f64 / 6.0_f64 * t56 * t60 * t87126 + 5.0_f64 / 162.0_f64 * t44 * t46065 * t87145 - t46090;
    t87155
}
