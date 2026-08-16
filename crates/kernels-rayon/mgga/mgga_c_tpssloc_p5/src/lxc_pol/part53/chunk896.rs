//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 896/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk896(t5: f64, t32244: f64, t9239: f64, t33: f64, t8705: f64, t2240: f64, t20: f64, t60: f64, t131: f64, t8308: f64, t8302: f64, t31000: f64, t31006: f64, t31013: f64, t31024: f64, t8707: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t32245 = t9239 * t32244;
    let t32248 = t33 * t8705;
    let t32249 = t2240 * t32248;
    let t32253 = 1.0_f64 / t60 / t20;
    let t32255 = t32253 * t131 * t8308;
    let t32257 = 20.0_f64 / 27.0_f64 * t8302 * t32255;
    let t32258 = t2240 * t32244;
    let t32262 = piecewise3(t8, 0.0_f64, 5.0_f64 / 36.0_f64 * t31000 * t8707 - 5.0_f64 / 6.0_f64 * t32245 * t31006 - 5.0_f64 / 9.0_f64 * t32249 * t31013 - t32257 + 5.0_f64 / 18.0_f64 * t32258 * t31024);
    (t32245, t32248, t32249, t32253, t32255, t32257, t32258, t32262)
}
