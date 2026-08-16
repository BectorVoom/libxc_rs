//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3327/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3327(t14353: f64, t14468: f64, t18850: f64, t198: f64, t207: f64, t2403: f64, t2430: f64, t40067: f64, t40072: f64, t40167: f64, t40171: f64, t40184: f64, t4343: f64, t4546: f64, t61358: f64, t61387: f64, t61429: f64, t62269: f64, t62270: f64, t62273: f64, t62518: f64, t62545: f64, t63055: f64, t63088: f64, t63129: f64, t892: f64) -> f64 {
    let t63145 = t198 * t207 * (t61358 + t61387 + t61429 + t62518 + t62545 + t63055 + t63088 + t63129) * t892 + t40067 - t40072 + 12.0_f64 * t2403 * t14353 * t4343 + t62269 + 3.0_f64 * t2403 * t18850 * t2430 + t40167 - t40171 - t62270 - t40184 + t62273 + 6.0_f64 * t2403 * t4546 * t14468;
    t63145
}
