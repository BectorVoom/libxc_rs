//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1151/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1151(t116: f64, t786: f64, t9534: f64, t39568: f64, t761: f64, t39382: f64, t39302: f64, t6589: f64, t68: f64, t236: f64, t40931: f64, t240: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41214 = t9534 * t786 * t116;
    let t41254 = 0.14035736694323150897e2_f64 * t761 * t39568;
    let t41258 = 0.91082604192152556044e5_f64 * t761 * t39382;
    let t41262 = 0.5848223622634646207e0_f64 * t761 * t39302;
    let t41315 = t68 * t6589;
    let t41347 = t40931 * t236;
    let t41349 = t812 * t41347 * t240;
    (t41214, t41254, t41258, t41262, t41315, t41349)
}
