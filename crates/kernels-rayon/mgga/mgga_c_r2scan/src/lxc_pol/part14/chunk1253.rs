//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1253/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1253(t39260: f64, t37377: f64, t37380: f64, t39054: f64, t41192: f64, t41196: f64, t41199: f64, t41201: f64, t41205: f64, t41208: f64, t41211: f64, t41213: f64, t41216: f64, t41219: f64, t41221: f64) -> f64 {
    let t42170 = 0.162600798888400151e-2_f64 * t39260;
    let t42171 = 0.1921128438866447784e-2_f64 * t37377 - 0.81300399444200075499e-3_f64 * t37380 + t41192 - t41196 - t41199 - t41201 - t41205 - t41208 - t39054 + t41211 - t41213 + t41216 - t42170 + t41219 - t41221;
    t42171
}
