//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 523/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk523(t352: f64, t6557: f64, t321: f64, t6522: f64, t333: f64, t305: f64, t326: f64, t3814: f64, t3839: f64, t4669: f64, t5148: f64, t5162: f64, t5259: f64, t5266: f64, t5271: f64, t5942: f64, t5945: f64, t5954: f64, t5957: f64, t5963: f64, t6308: f64, t6311: f64, t6315: f64, t6332: f64, t6335: f64, t6339: f64, t6382: f64, t6387: f64, t6482: f64, t6523: f64, t6530: f64, t797: f64, t838: f64) -> (f64, f64, f64, f64) {
    let t6558 = t6557 * t352;
    let t6561 = t6522 * t321;
    let t6564 = t6522 * t333;
    let t6567 = -0.23948483403727617128e0_f64 * t5148 * t6523 + 0.71845450211182851384e0_f64 * t5271 * t6382 - 0.14369090042236570277e1_f64 * t5162 * t6387 - 0.35922725105591425692e0_f64 * t4669 * t6530 - 0.11974241701863808564e0_f64 * t326 * t5945 - 0.59871208509319042821e-1_f64 * t326 * t6339 - 0.11974241701863808564e0_f64 * t305 * t5963 - 0.11974241701863808564e0_f64 * t326 * t6308 - 0.71845450211182851384e0_f64 * t3814 * t5954 + 0.11974241701863808564e1_f64 * t3839 * t5957 + 0.17961362552795712846e0_f64 * t797 * t6332 - 0.23948483403727617128e0_f64 * t838 * t6335 + 0.11974241701863808564e0_f64 * t305 * t6482 + 0.35922725105591425692e0_f64 * t797 * t6311 + 0.35922725105591425692e0_f64 * t797 * t6315 + 0.11974241701863808564e0_f64 * t305 * t5942 + 0.23948483403727617128e0_f64 * t5266 * t6558 + 0.23948483403727617128e0_f64 * t5259 * t6561 - 0.35922725105591425692e0_f64 * t4669 * t6564;
    (t6558, t6561, t6564, t6567)
}
