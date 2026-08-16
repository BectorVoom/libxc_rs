//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1021/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1021(t3814: f64, t39670: f64, t3851: f64, t39692: f64, t3826: f64, t40920: f64, t25640: f64, t38564: f64, t2115: f64, t41056: f64, t41044: f64, t2100: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41336 = t3814 * t39670;
    let t41338 = t3851 * t39692;
    let t41340 = t3826 * t39692;
    let t41341 = 0.10620923284048465071e-1_f64 * t41340;
    let t41342 = t3814 * t40920;
    let t41344 = t25640 * t38564;
    let t41347 = t2115 * t41056;
    let t41348 = 0.4838420607177634088e-3_f64 * t41347;
    let t41349 = t2115 * t41044;
    let t41351 = t2100 * t41044;
    (t41336, t41338, t41341, t41342, t41344, t41348, t41349, t41351)
}
