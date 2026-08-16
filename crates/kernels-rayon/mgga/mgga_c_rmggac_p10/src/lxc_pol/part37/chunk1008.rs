//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1008/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1008(t27101: f64, t78220: f64, t14444: f64, t1627: f64, t25854: f64, t76479: f64, t5148: f64, t570: f64, t71903: f64, t321: f64, t77970: f64, t77204: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78222 = 0.23948483403727617128e0_f64 * t27101 * t78220;
    let t78223 = t14444 * t1627;
    let t78225 = 0.35922725105591425692e0_f64 * t25854 * t78223;
    let t78228 = 0.54549323308490683456e-1_f64 * t76479;
    let t78236 = t5148 * t71903 * t570;
    let t78237 = 0.2993560425465952141e-1_f64 * t78236;
    let t78240 = 0.11974241701863808564e0_f64 * t5148 * t77970 * t321;
    let t78244 = t27101 * t77204;
    (t78222, t78223, t78225, t78228, t78237, t78240, t78244)
}
