//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2325/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2325(t20816: f64, t2427: f64, t46369: f64, t46371: f64, t46376: f64, t58984: f64, t41259: f64, t46433: f64, t39593: f64, t41254: f64, t41258: f64, t41262: f64, t46336: f64, t67472: f64, t67475: f64, t67478: f64, t67480: f64, t67482: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t67484 = 4.0_f64 * t2427 * t20816;
    let t67485 = 12.0_f64 * t46369;
    let t67486 = 0.65061487801810439052e-1_f64 * t46371;
    let t67487 = 0.17544670867903938621e1_f64 * t46376;
    let t67488 = 0.73245789224026180216e-3_f64 * t58984;
    let t67489 = 0.5848223622634646207e0_f64 * t41259;
    let t67490 = 0.17090684152272775384e-2_f64 * t46433;
    let t67491 = t67472 + t67475 + t67478 + t67480 + t46336 - t39593 + t67482 + t67484 + t67485 - t67486 + t41254 - t67487 + t67488 - t41258 - t67489 - t41262 - t67490;
    (t67484, t67485, t67486, t67487, t67488, t67489, t67490, t67491)
}
