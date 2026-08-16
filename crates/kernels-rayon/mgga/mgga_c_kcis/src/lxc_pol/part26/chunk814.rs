//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 814/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk814(t1507: f64, t456: f64, t1444: f64, t1455: f64, t1523: f64, t318: f64, t86: f64, t334: f64, t565: f64, t3754: f64, t1520: f64, t752: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12361 = t1507 * t456;
    let t12371 = t1455 * t1444;
    let t12397 = t86 * t318 * t1523;
    let t12401 = 0.11791604938271604938e-1_f64 * t86 * t334 * t565;
    let t12406 = t1455 * t3754;
    let t12431 = t752 * t1520;
    (t12361, t12371, t12397, t12401, t12406, t12431)
}
