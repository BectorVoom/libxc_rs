//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 808/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk808(t8651: f64, t38274: f64, t38275: f64, t38276: f64, t38277: f64, t38278: f64, t38279: f64, t7545: f64, t7550: f64, t8653: f64, t9381: f64, t7670: f64, t7674: f64, t7679: f64, t7681: f64, t7684: f64, t7686: f64, t7689: f64, t7693: f64, t7698: f64, t8673: f64, t9412: f64) -> (f64, f64) {
    let t38280 = 0.20455996240684006296e-1_f64 * t8651;
    let t38282 = t38274 - t38275 + t38276 + t38277 + t38278 - t38279 - t38280 - 0.25538759935978703638e-4_f64 * t8653 - t9381 + t7545 + t7550;
    let t38290 = -t7670 + 0.72732431077987577942e-1_f64 * t8673 + t7674 - t7679 - t7681 - t7684 - t7686 - t7689 - t7693 - t7698 - t9412;
    (t38282, t38290)
}
