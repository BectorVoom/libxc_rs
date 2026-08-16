//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3230/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3230(t11075: f64, t14468: f64, t1544: f64, t18268: f64, t18850: f64, t198: f64, t2393: f64, t2394: f64, t2403: f64, t2430: f64, t4541: f64, t4542: f64, t49950: f64, t5966: f64, t61234: f64, t61240: f64, t61244: f64, t61245: f64, t61248: f64, t61249: f64, t61250: f64, t61261: f64) -> f64 {
    let t61262 = 6.0_f64 * t11075 * t4541 * t5966 + 12.0_f64 * t14468 * t4541 * t4542 + 6.0_f64 * t1544 * t2403 * t49950 - 3.0_f64 * t18268 * t2403 * t2430 + 6.0_f64 * t18850 * t2394 * t4541 + 12.0_f64 * t198 * t2393 * t61234 - t61240 + t61244 + t61245 + t61248 + t61249 + t61250 + t61261;
    t61262
}
