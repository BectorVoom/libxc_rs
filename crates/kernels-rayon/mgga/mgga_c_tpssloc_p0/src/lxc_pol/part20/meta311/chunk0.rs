//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1566/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1566(t11129: f64, t3403: f64, t11135: f64, t11203: f64, t11161: f64, t11170: f64, t11197: f64, t11200: f64, t11206: f64, t11209: f64, t11211: f64, t11213: f64, t11215: f64, t11217: f64, t11221: f64, t11224: f64) -> (f64, f64, f64, f64) {
    let t11366 = t11129 * t3403;
    let t11369 = 0.93932222222222222223e0_f64 * t11135;
    let t11372 = 0.36793333333333333333e0_f64 * t11203;
    let t11383 = -t11369 - 0.3883875e1_f64 * t11197 + 0.247573125e0_f64 * t11200 - t11372 + 0.49671e0_f64 * t11206 + 0.82785e-1_f64 * t11209 + 0.27595e0_f64 * t11211 + 0.5519e-1_f64 * t11213 - 0.33114e0_f64 * t11215 - 0.16557e0_f64 * t11217 + 0.36793333333333333333e-1_f64 * t11221 - 0.16557e0_f64 * t11224 - 0.60384999999999999999e0_f64 * t11161 + 0.181155e1_f64 * t11170;
    (t11366, t11369, t11372, t11383)
}
