//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1555/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1555(t1264: f64, t20272: f64, t247: f64, t5405: f64, t6429: f64, t3626: f64, t6425: f64, t1794: f64, t5245: f64, t1250: f64, t3720: f64, t140: f64, t6652: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21153 = t247 * t1264 * t20272;
    let t21156 = t6429 * t5405;
    let t21157 = t3626 * t21156;
    let t21160 = t6425 * t5405;
    let t21161 = t3626 * t21160;
    let t21164 = t5245 * t1794;
    let t21165 = t21164 * t1250;
    let t21166 = t3720 * t21165;
    let t21169 = t140 * t6652;
    (t21153, t21157, t21161, t21164, t21166, t21169)
}
