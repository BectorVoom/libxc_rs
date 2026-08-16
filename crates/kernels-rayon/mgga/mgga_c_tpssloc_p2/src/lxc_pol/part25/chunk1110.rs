//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1110/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1110(t22699: f64, t22704: f64, t22705: f64, t1351: f64, t1992: f64, t3879: f64, t550: f64, t6976: f64, t22741: f64, t22696: f64, t6914: f64, t552: f64) -> (f64, f64, f64, f64, f64) {
    let t81115 = t22704 * t22705 * t22699;
    let t81122 = t1992 * t6976 * t3879 * t1351 * t550;
    let t81125 = t22704 * t22705 * t22741;
    let t81127 = t6914 * t22696;
    let t81129 = t552 * t3879;
    (t81115, t81122, t81125, t81127, t81129)
}
