//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1191/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1191(t11199: f64, t3275: f64, t7040: f64, t3579: f64, t38678: f64, t11189: f64, t40464: f64, t3262: f64, t3472: f64, t40416: f64, t11020: f64, t12086: f64) -> (f64, f64, f64, f64, f64) {
    let t41211 = t3275 * t11199 * t7040 / 2.0_f64;
    let t41213 = t3579 * t38678 / 4.0_f64;
    let t41216 = 45.0_f64 / 64.0_f64 * t3275 * t11189 * t40464;
    let t41219 = 15.0_f64 / 8.0_f64 * t3262 * t3472 * t40416;
    let t41221 = t11020 * t12086 / 4.0_f64;
    (t41211, t41213, t41216, t41219, t41221)
}
