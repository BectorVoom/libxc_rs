//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 336/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk336(t3191: f64, t650: f64, t2211: f64, t664: f64, t118: f64, t3088: f64, t3095: f64, t3097: f64, t3091: f64, t3100: f64, t3103: f64) -> (f64, f64, f64, f64) {
    let t3192 = t3191 * t650;
    let t3193 = 0.34093327067806677161e-2_f64 * t3192;
    let t3194 = t2211 * t664;
    let t3195 = t118 * t3194;
    let t3196 = 0.39914139006212695214e-1_f64 * t3195;
    let t3197 = 0.49892673757765869017e-2_f64 * t3088;
    let t3199 = 0.10354269702074620472e-2_f64 * t3095;
    let t3200 = 0.16595192631325726674e-3_f64 * t3097;
    let t3203 = t3197 - 0.34093327067806677161e-2_f64 * t3091 + t3199 + t3200 - 0.90720386384580639149e-4_f64 * t3100 + 0.24108102678124669848e-4_f64 * t3103;
    (t3193, t3194, t3196, t3203)
}
