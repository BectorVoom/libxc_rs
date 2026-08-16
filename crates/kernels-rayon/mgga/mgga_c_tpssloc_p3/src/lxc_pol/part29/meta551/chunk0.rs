//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1950/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1950(t27496: f64, t27497: f64, t5083: f64, t7376: f64, t7375: f64, t1419: f64, t6794: f64, t131: f64, t467: f64, t5075: f64, t225: f64, t8034: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27498 = t27496 * t27497;
    let t27501 = t5083 * t7376;
    let t27502 = t7375 * t27501;
    let t27505 = t1419 * t6794;
    let t27506 = t27505 * t131;
    let t27507 = t27506 * t467;
    let t27510 = t5075 * t7376;
    let t27511 = t7375 * t27510;
    let t27516 = t8034 * t225;
    (t27498, t27501, t27502, t27505, t27506, t27507, t27510, t27511, t27516)
}
