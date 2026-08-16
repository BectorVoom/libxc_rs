//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1247/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1247(t3701: f64, t7939: f64, t8639: f64, t1390: f64, t601: f64, t9238: f64, t85: f64, t24: f64, t12019: f64, t566: f64, t3700: f64, t2751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33899 = t3701 * t7939;
    let t36740 = t3701 * t8639;
    let t37790 = t8639 * t1390;
    let t39054 = t601 * t9238;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = 1.0_f64 / t12019 / t566;
    let t40610 = t3700 * t3700;
    let t40611 = 1.0_f64 / t40610;
    let t40771 = t2751 * t2751;
    (t33899, t36740, t37790, t39054, t39063, t40590, t40611, t40771)
}
