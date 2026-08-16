//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3143/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3143(t1168: f64, t12423: f64, t12429: f64, t12430: f64, t12486: f64, t12487: f64, t12504: f64, t12508: f64, t12511: f64, t16948: f64, t16959: f64, t17023: f64, t17032: f64, t17085: f64, t17086: f64, t1745: f64, t1756: f64, t1757: f64, t3452: f64, t3477: f64, t3479: f64, t45075: f64, t45188: f64, t45190: f64, t45194: f64, t5125: f64, t5147: f64, t56268: f64, t56271: f64, t56275: f64, t56277: f64) -> f64 {
    let t57943 = -6.0_f64 * t17023 * t12504 + 0.96491876992155210402e2_f64 * t17032 * t12508 - 6.0_f64 * t45194 * t5125 + 0.96491876992155210402e2_f64 * t45075 * t5147 - 12.0_f64 * t12511 * t16948 + 0.19298375398431042081e3_f64 * t12423 * t16959 - 6.0_f64 * t3452 * t17086 * t1168 + 0.96491876992155210402e2_f64 * t3477 * t17085 * t3479 * t1168 - 0.14035736694323150897e2_f64 * t12486 * t1757 * t12487 - 24.0_f64 * t12429 * t1745 * t12430 + 0.91082604192152556044e5_f64 * t45188 * t1756 * t45190 * t12487 - t56268 - t56271 - t56275 + t56277;
    t57943
}
