//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2240/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2240(t40: f64, t4199: f64, t9713: f64, t41255: f64, t41259: f64, t41265: f64, t1471: f64, t31: f64, t9898: f64, t10913: f64, t12606: f64, t12950: f64, t1430: f64, t2244: f64, t2250: f64, t4007: f64, t4010: f64, t4104: f64, t45872: f64, t607: f64, t75: f64, t767: f64, t9258: f64, t9288: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t46376 = t4199 * t9713;
    let t46377 = 0.5848223622634646207e0_f64 * t46376;
    let t46384 = 0.17544670867903938621e1_f64 * t41255;
    let t46385 = 0.17544670867903938621e1_f64 * t41259;
    let t46386 = 0.5848223622634646207e0_f64 * t41265;
    let t46387 = t31 * t1471;
    let t46389 = 24.0_f64 * t46387 * t9898;
    let t46407 = piecewise3(t146, 0.0_f64, -56.0_f64 / 81.0_f64 * t4007 * t9288 + 8.0_f64 / 9.0_f64 * t4010 * t2244 + 8.0_f64 / 9.0_f64 * t1430 * t10913 - 2.0_f64 / 3.0_f64 * t75 * t12606 * t607 - 2.0_f64 / 3.0_f64 * t12950 * t2250 - 2.0_f64 / 9.0_f64 * t4104 * t9258 + 2.0_f64 / 3.0_f64 * t767 * t45872);
    (t46377, t46384, t46385, t46386, t46389, t46407)
}
