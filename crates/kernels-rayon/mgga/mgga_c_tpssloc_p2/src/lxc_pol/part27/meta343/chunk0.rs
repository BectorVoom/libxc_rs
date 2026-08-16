//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1424/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1424(t225: f64, t3755: f64, t3700: f64, t570: f64, t1390: f64, t3914: f64, t3698: f64, t3701: f64, t112: f64, t3931: f64, t111: f64, t1395: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12444 = t3755 * t225;
    let t12461 = 1.0_f64 / t3700 / t570;
    let t12466 = t3914 * t1390;
    let t12477 = t3698 * t3701;
    let t12521 = t3931 * t112;
    let t12524 = t1395 * t111;
    (t12444, t12461, t12466, t12477, t12521, t12524)
}
