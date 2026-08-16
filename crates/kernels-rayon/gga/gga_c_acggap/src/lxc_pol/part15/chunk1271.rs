//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1271/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1271(t315: f64, t323: f64, t9971: f64, t2230: f64, t40653: f64, t2146: f64, t2147: f64, t33274: f64, t33278: f64, t33281: f64, t33284: f64, t33286: f64, t33293: f64, t38092: f64, t38430: f64, t38432: f64, t38441: f64, t38443: f64, t38453: f64, t556: f64, t8400: f64, t8402: f64, t9367: f64) -> f64 {
    let t42220 = t315 * t9971 * t323;
    let t42222 = t40653 * t2230;
    let t42225 = 0.8673628188205199462e0_f64 * t33274 - 0.8673628188205199462e0_f64 * t33278 + 0.8673628188205199462e0_f64 * t33281 - t33284 + 0.17347256376410398924e1_f64 * t2146 * t2147 * t9367 * t556 - 0.34694512752820797848e1_f64 * t33286 - t38430 - 0.17347256376410398924e1_f64 * t38432 + 0.8673628188205199462e0_f64 * t8400 * t38092 * t8402 - t33293 + t38441 + 0.17347256376410398924e1_f64 * t38443 - 0.65854491829355115987e0_f64 * t42220 + 0.8673628188205199462e0_f64 * t42222 + 0.34694512752820797848e1_f64 * t38453;
    t42225
}
