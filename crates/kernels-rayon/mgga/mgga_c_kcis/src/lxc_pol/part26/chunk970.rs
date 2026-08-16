//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 970/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk970(t22289: f64, t4163: f64, t4162: f64, t16771: f64, t17277: f64, t22261: f64, t22263: f64, t22266: f64, t22268: f64, t22273: f64, t22277: f64, t22280: f64, t22282: f64, t22287: f64) -> (f64, f64, f64) {
    let t22290 = t4163 * t22289;
    let t22291 = t4162 * t22290;
    let t22292 = t16771 * t22291;
    let t22294 = 0.66327777777777777776e-2_f64 * t22261 + t17277 - 0.22109259259259259259e-2_f64 * t22263 - 0.14739506172839506172e-1_f64 * t22266 + 0.22109259259259259258e-2_f64 * t22268 - 0.44218518518518518516e-2_f64 * t22273 + 0.33163888888888888888e-2_f64 * t22277 + 0.88437037037037037033e-2_f64 * t22280 - 0.33163888888888888888e-2_f64 * t22282 - 0.33163888888888888888e-2_f64 * t22287 + 0.66327777777777777776e-2_f64 * t22292;
    (t22290, t22292, t22294)
}
