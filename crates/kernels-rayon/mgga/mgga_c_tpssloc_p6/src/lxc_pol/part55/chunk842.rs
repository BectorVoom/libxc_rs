//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 842/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk842(t3036: f64, t67: f64, t1176: f64, t134: f64, t1239: f64, t68: f64, t3639: f64, t500: f64, t1376: f64, t1995: f64, t246: f64, t3700: f64, t570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10401 = t3036 * t67;
    let t11588 = t134 * t1176;
    let t11604 = t1239 * t1239;
    let t11605 = 1.0_f64 / t11604;
    let t11606 = t68 * t11605;
    let t11947 = 1.0_f64 / t3639 / t500;
    let t12019 = t1376 * t1376;
    let t12020 = 1.0_f64 / t12019;
    let t12021 = t68 * t12020;
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12461 = 1.0_f64 / t3700 / t570;
    (t10401, t11588, t11604, t11605, t11606, t11947, t12019, t12020, t12021, t12419, t12461)
}
