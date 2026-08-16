//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1038/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1038(t136308: f64, t6449: f64, t136303: f64, t22513: f64, t1554: f64, t938: f64, t136367: f64, t32146: f64, t6441: f64, t25749: f64, t32247: f64, t101075: f64, t136275: f64, t136279: f64, t136301: f64, t136305: f64, t136336: f64, t136469: f64, t25709: f64, t25746: f64, t25787: f64, t25839: f64, t32233: f64, t32239: f64, t34468: f64, t93117: f64) -> (f64, f64, f64, f64, f64) {
    let t145071 = t136308 * t6449;
    let t145074 = t136303 * t6449;
    let t145075 = t22513 * t145074;
    let t145077 = t1554 * t938;
    let t145099 = t32146 * t136367 * t6441;
    let t145108 = t32247 * t25749;
    let t145120 = -0.23754828622903245155e-2_f64 * t32146 * t136336 * t6441 + 0.29693535778629056444e-3_f64 * t145099 + 0.51074886703703703704e-1_f64 * t32247 * t136469 * t25709 - 0.85124811172839506173e-2_f64 * t136279 + 0.68099848938271604939e-1_f64 * t32247 * t25746 - 0.45497819271775541929e-4_f64 * t136301 - 0.85124811172839506173e-2_f64 * t145108 - 0.13623313276722699538e-2_f64 * t93117 * t32233 * t25839 + 0.37842536879785276493e-4_f64 * t136305 + 0.20434969915084049306e-2_f64 * t101075 * t32233 * t25787 + 0.36398255417420433543e-3_f64 * t32239 * t136275 * t34468;
    (t145071, t145074, t145075, t145077, t145120)
}
