//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 493/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk493(t3869: f64, t3903: f64, t1216: f64, t1349: f64, t1402: f64, t338: f64, t3729: f64, t3814: f64, t3815: f64, t3817: f64, t3819: f64, t3820: f64, t3824: f64, t3827: f64, t3832: f64, t3835: f64, t417: f64, t451: f64) -> (f64, f64) {
    let t3904 = t3869 + t3903;
    let t3906 = t3814 + 0.46853067927761790996e-2_f64 * t3815 + 0.93706135855523581992e-2_f64 * t3817 + 0.46853067927761790996e-2_f64 * t3819 * t3820 + 0.93706135855523581992e-2_f64 * t1349 * t3824 - 0.23426533963880895498e-2_f64 * t1349 * t3827 + 0.14055920378328537299e-1_f64 * t417 * t3832 - 0.46853067927761790996e-2_f64 * t417 * t3835 - t3729 * t451 - 2.0_f64 * t1216 * t1402 - t338 * t3904;
    (t3904, t3906)
}
