//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1055/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1055(t21488: f64, t795: f64, t805: f64, t2571: f64, t5397: f64, t2101: f64, t165: f64, t2089: f64, t1865: f64, t935: f64, t16788: f64, t278: f64, t481: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21490 = t21488 * t805 * t795;
    let t21491 = t2571 * t5397;
    let t21497 = t21488 * t805 * t2101;
    let t21502 = t165 * t2089;
    let t21503 = t935 * t1865;
    let t21504 = t21502 * t21503;
    let t21556 = t481 * t16788 * t278;
    (t21490, t21491, t21497, t21502, t21503, t21504, t21556)
}
