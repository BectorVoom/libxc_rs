//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1208/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1208(t1458: f64, t1868: f64, t3701: f64, t7752: f64, t576: f64, t22811: f64, t85: f64, t24: f64, t12019: f64, t566: f64, t68: f64, t3700: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33085 = t1868 * t1458;
    let t33136 = t3701 * t7752;
    let t33185 = t576 * t1458;
    let t39041 = 1.0_f64 / t22811;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = 1.0_f64 / t12019 / t566;
    let t40591 = t68 * t40590;
    let t40610 = t3700 * t3700;
    (t33085, t33136, t33185, t39041, t39063, t40591, t40610)
}
