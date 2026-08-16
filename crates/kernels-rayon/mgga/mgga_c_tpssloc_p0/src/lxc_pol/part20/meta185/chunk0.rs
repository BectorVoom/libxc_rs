//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1134/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1134(t4483: f64, t961: f64, t1589: f64, t2940: f64, t1580: f64, t2904: f64, t952: f64, t959: f64, t4471: f64, t942: f64, t951: f64, t2929: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4485 = 0.5848223622634646207e0_f64 * t4483 * t961;
    let t4487 = 0.5848223622634646207e0_f64 * t2940 * t1589;
    let t4488 = t2904 * t1580;
    let t4489 = t4488 * t952;
    let t4491 = 0.11696447245269292414e1_f64 * t959 * t4489;
    let t4493 = t942 * t4471 * t951;
    let t4495 = 0.5848223622634646207e0_f64 * t959 * t4493;
    let t4496 = t2929 * t1580;
    (t4485, t4487, t4488, t4489, t4491, t4493, t4495, t4496)
}
