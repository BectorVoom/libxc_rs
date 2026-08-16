//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3238/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3238(t13302: f64, t13324: f64, t1480: f64, t18281: f64, t21732: f64, t21755: f64, t21758: f64, t22671: f64, t22688: f64, t22692: f64, t2275: f64, t2282: f64, t4186: f64, t4201: f64, t4210: f64, t4211: f64, t44: f64, t46065: f64, t46074: f64, t56: f64, t5825: f64, t5843: f64, t606: f64, t614: f64) -> f64 {
    let t85295 = 5.0_f64 / 162.0_f64 * t56 * t46074 * t22688 * t606 + 5.0_f64 / 6.0_f64 * t56 * t13324 * t5825 + 5.0_f64 / 6.0_f64 * t56 * t4210 * t18281 + 5.0_f64 / 18.0_f64 * t56 * t2282 * t22671 * t606 - 5.0_f64 / 36.0_f64 * t44 * t21732 * t4186 + 5.0_f64 / 162.0_f64 * t44 * t46065 * t22688 * t606 + 5.0_f64 / 6.0_f64 * t44 * t13302 * t5825 + 5.0_f64 / 6.0_f64 * t44 * t4201 * t18281 + 5.0_f64 / 18.0_f64 * t44 * t2275 * t22671 * t606 + 220.0_f64 / 27.0_f64 * t5843 * t4211 - 40.0_f64 / 9.0_f64 * t1480 * t21758 - 10.0_f64 / 27.0_f64 * t1480 * t21755 - 20.0_f64 / 9.0_f64 * t614 * t22692;
    t85295
}
