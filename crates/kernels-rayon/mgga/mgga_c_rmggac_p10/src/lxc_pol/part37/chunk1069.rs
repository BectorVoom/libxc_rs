//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1069/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1069(t570: f64, t73450: f64, t1356: f64, t15014: f64, t534: f64, t71300: f64, t72: f64, t75002: f64, t75003: f64, t75005: f64, t77357: f64, t77361: f64, t77362: f64, t77363: f64, t77365: f64, t77367: f64, t77370: f64, t77372: f64, t77374: f64, t77376: f64, t77379: f64) -> (f64, f64) {
    let t80192 = t73450 * t570;
    let t80197 = t77357 - t77361 + t77362 + t77363 + t77365 + t77367 + t77370 + t77372 + t77374 + 0.39914139006212695214e-1_f64 * t1356 * t80192 + t72 * t534 * t15014 - t75002 - t71300 - t77376 - t77379 - t75003 + t75005;
    (t80192, t80197)
}
