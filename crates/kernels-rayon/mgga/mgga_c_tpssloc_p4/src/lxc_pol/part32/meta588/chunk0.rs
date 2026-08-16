//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1976/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1976(t85: f64, t24: f64, t12019: f64, t566: f64, t3700: f64, t2751: f64, t10108: f64, t257: f64, t3639: f64, t11604: f64, t496: f64, t1406: f64, t9238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = 1.0_f64 / t12019 / t566;
    let t40610 = t3700 * t3700;
    let t40611 = 1.0_f64 / t40610;
    let t40771 = t2751 * t2751;
    let t40772 = 1.0_f64 / t40771;
    let t40889 = 1.0_f64 / t10108 / t257;
    let t43705 = t3639 * t3639;
    let t43706 = 1.0_f64 / t43705;
    let t45349 = 1.0_f64 / t11604 / t496;
    let t45844 = t1406 * t9238;
    (t39063, t40590, t40611, t40772, t40889, t43706, t45349, t45844)
}
