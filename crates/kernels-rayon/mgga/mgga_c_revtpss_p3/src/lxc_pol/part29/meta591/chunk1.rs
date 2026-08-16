//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1964/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1964(t102244: f64, t94669: f64, t102225: f64, t102237: f64, t102239: f64, t102241: f64, t7528: f64, t96243: f64, t96246: f64, t96249: f64, t96253: f64, t96257: f64, t96260: f64, t96262: f64, t96265: f64, t98050: f64) -> f64 {
    let t102246 = 0.15421710918628844644e0_f64 * t94669 * t102244;
    let t102248 = -0.3427046870806409921e-2_f64 * t102225 - 0.14456046980341999104e-1_f64 * t96243 - 0.34270468708064099208e-1_f64 * t96246 + 0.12851425765524037203e-1_f64 * t96249 - 0.13009920719177044025e-2_f64 * t96253 + 0.8673628188205199462e0_f64 * t98050 * t7528 + t102237 - t102239 + t102241 - t96257 - 0.45699670022203476294e-2_f64 * t96260 - 0.12851425765524037203e-1_f64 * t96262 - t102246 - 0.68540937416128198416e-1_f64 * t96265;
    t102248
}
