//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1030/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1030(t1992: f64, t33249: f64, t90566: f64, t115447: f64, t120383: f64, t124139: f64, t124142: f64, t124146: f64, t127252: f64, t127254: f64, t127256: f64, t127258: f64, t127263: f64, t127267: f64, t127270: f64, t127273: f64) -> (f64, f64) {
    let t128604 = t1992 * t90566 * t33249;
    let t128616 = -t127252 / 192.0_f64 - t127254 / 96.0_f64 + 5.0_f64 / 192.0_f64 * t127256 + t124139 + t127258 / 96.0_f64 + t124142 - 0.96894614625936938046e-2_f64 * t127263 + 0.67826230238155856632e-1_f64 * t127267 + 0.19378922925187387609e-1_f64 * t127270 - t115447 - 0.16149102437656156341e-2_f64 * t127273 + t124146 + 0.13565246047631171327e0_f64 * t120383;
    (t128604, t128616)
}
