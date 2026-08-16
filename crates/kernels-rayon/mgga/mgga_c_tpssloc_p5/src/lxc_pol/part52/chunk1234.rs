//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1234/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1234(t3701: f64, t8488: f64, t12461: f64, t8492: f64, t1390: f64, t601: f64, t9238: f64, t85: f64, t24: f64, t12019: f64, t566: f64, t3700: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36363 = t3701 * t8488;
    let t36533 = t12461 * t8492;
    let t37589 = t8488 * t1390;
    let t37593 = t8492 * t3701;
    let t39054 = t601 * t9238;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = 1.0_f64 / t12019 / t566;
    let t40610 = t3700 * t3700;
    (t36363, t36533, t37589, t37593, t39054, t39063, t40590, t40610)
}
