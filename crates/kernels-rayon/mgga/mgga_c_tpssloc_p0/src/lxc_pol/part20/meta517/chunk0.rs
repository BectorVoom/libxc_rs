//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2042/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2042(t12088: f64, t2528: f64, t3691: f64, t9919: f64, t2367: f64, t2508: f64, t39378: f64, t9493: f64, t1294: f64, t9713: f64, t2405: f64, t2412: f64, t9479: f64, t9481: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39530 = t12088 * t2528;
    let t39532 = t3691 * t9919;
    let t39535 = 1.0_f64 / t2508 / t2367;
    let t39537 = t39535 * t39378 * t9493;
    let t39539 = 0.12304822629859687989e5_f64 * t1294 * t39537;
    let t39540 = t3691 * t9713;
    let t39549 = 0.3103560775156404018e4_f64 * t9479 * t2412 * t9481 * t2405;
    (t39530, t39532, t39535, t39537, t39539, t39540, t39549)
}
