//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 910/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk910(t11729: f64, t69507: f64, t11732: f64, t69433: f64, t12140: f64, t69176: f64, t305: f64, t76062: f64, t75674: f64, t793: f64, t5259: f64, t75515: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76285 = t69507 * t11729;
    let t76287 = t69433 * t11732;
    let t76289 = t69176 * t12140;
    let t76291 = t305 * t76062;
    let t76292 = 0.79828278012425390427e-1_f64 * t76291;
    let t76305 = t793 * t75674;
    let t76310 = t5259 * t75515;
    (t76285, t76287, t76289, t76292, t76305, t76310)
}
