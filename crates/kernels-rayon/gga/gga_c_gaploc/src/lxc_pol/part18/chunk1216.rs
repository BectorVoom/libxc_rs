//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1216/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1216(t10773: f64, t7137: f64, t21556: f64, t3448: f64, t10722: f64, t10776: f64, t10790: f64, t11083: f64, t1841: f64, t1897: f64, t2042: f64, t2508: f64, t2580: f64, t32207: f64, t32213: f64, t32215: f64, t32222: f64, t32226: f64, t32230: f64, t32234: f64, t32241: f64, t3433: f64, t5288: f64, t5293: f64, t702: f64, t7129: f64, t7226: f64, t7289: f64) -> f64 {
    let t32243 = 0.20508069947045931424e-1_f64 * t7137 * t10773;
    let t32245 = 0.41016139894091862846e-1_f64 * t21556 * t3448;
    let t32248 = 0.76905262301422242837e-2_f64 * t2508 * t2042 * t3433 + t32207 + 0.20508069947045931424e-1_f64 * t5293 * t10722 + t32213 - 0.34180116578409885705e-2_f64 * t1841 * t7289 * t32215 + t32222 + t32226 - 0.15381052460284448567e-1_f64 * t1897 * t11083 * t702 + 0.92286314761706691403e-1_f64 * t2508 * t2580 * t32230 - 0.46143157380853345701e-1_f64 * t2508 * t7226 * t32234 - 0.15381052460284448567e-1_f64 * t5288 * t10776 + t32241 + t32243 + t32245 - 0.46143157380853345702e-1_f64 * t7129 * t10790;
    t32248
}
