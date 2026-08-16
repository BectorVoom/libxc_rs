//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1048/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1048(t14477: f64, t861: f64, t141: f64, t2464: f64, t4579: f64, t581: f64, t2459: f64, t2515: f64, t14452: f64, t835: f64, t128: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14478 = t861 * t14477;
    let t14479 = t141 * t14478;
    let t14481 = t2464 * t4579;
    let t14482 = t14481 * t581;
    let t14483 = t861 * t14482;
    let t14484 = t141 * t14483;
    let t14486 = t2459 * t4579;
    let t14487 = t14486 * t581;
    let t14488 = t2515 * t14487;
    let t14489 = t141 * t14488;
    let t14491 = t835 * t14452;
    let t14492 = t128 * t14491;
    (t14479, t14482, t14484, t14487, t14489, t14492)
}
