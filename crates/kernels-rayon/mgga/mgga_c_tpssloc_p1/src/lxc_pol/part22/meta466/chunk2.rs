//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1851/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1851(t20601: f64, t539: f64, t1842: f64, t6439: f64, t12021: f64, t6460: f64, t3887: f64, t553: f64, t12249: f64, t20490: f64, t20495: f64, t3897: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20602 = t539 * t20601;
    let t20608 = t6439 * t1842;
    let t20609 = t12021 * t20608;
    let t20612 = t1842 * t6460;
    let t20613 = t3887 * t20612;
    let t20616 = t553 * t20601;
    let t20622 = t12249 * t20490;
    let t20625 = t3897 * t20495;
    (t20602, t20608, t20609, t20613, t20616, t20622, t20625)
}
