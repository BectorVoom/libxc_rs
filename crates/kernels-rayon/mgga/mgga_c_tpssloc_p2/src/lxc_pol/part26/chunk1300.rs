//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1300/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1300(t6604: f64, t9971: f64, t1888: f64, t81672: f64, t9975: f64, t22996: f64, t2632: f64, t23110: f64, t23180: f64, t23185: f64, t206: f64, t22723: f64, t268: f64) -> (f64, f64, f64, f64) {
    let t82018 = t6604 * t9971;
    let t82021 = t1888 * t82018 * t81672 * t9975;
    let t82025 = t1888 * t22996 * t81672 * t2632;
    let t82028 = t23185 * t23110 * t23180;
    let t82031 = t22723 * t206 * t268;
    (t82021, t82025, t82028, t82031)
}
