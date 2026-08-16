//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1322/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1322(t14207: f64, t17964: f64, t14247: f64, t5559: f64, t14258: f64, t17960: f64, t4761: f64, t4766: f64, t14300: f64, t5552: f64, t4718: f64, t61057: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69985 = t17964 * t14207;
    let t69989 = t5559 * t14247;
    let t69991 = t5559 * t14258;
    let t69993 = t17960 * t4761;
    let t69995 = t17960 * t4766;
    let t69997 = t5552 * t14300;
    let t69999 = t61057 * t4718;
    (t69985, t69989, t69991, t69993, t69995, t69997, t69999)
}
