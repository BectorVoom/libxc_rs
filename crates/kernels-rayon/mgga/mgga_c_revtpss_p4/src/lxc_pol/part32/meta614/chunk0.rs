//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1954/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1954(t108278: f64, t7063: f64, t108277: f64, t1955: f64, t22307: f64, t786: f64, t1444: f64, t6874: f64, t6862: f64, t22107: f64, t26028: f64, t22111: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t108279 = t7063 * t108278;
    let t108282 = t1955 * t108277;
    let t108371 = t1955 * t22307;
    let t108379 = t786 * t108278;
    let t108448 = t6874 * t1444;
    let t108502 = t6862 * t1444;
    let t108508 = t26028 * t22107;
    let t108510 = t26028 * t22111;
    (t108279, t108282, t108371, t108379, t108448, t108502, t108508, t108510)
}
