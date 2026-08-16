//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 997/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk997(t4397: f64, t520: f64, t3273: f64, t4416: f64, t10106: f64, t13685: f64, t3260: f64, t4459: f64, t4415: f64, t3342: f64, t5424: f64, t1248: f64, t13671: f64, t774: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13754 = t520 * t4397;
    let t13756 = t3273 * t4416 * t13754;
    let t13760 = t3273 * t13685 * t10106;
    let t13763 = t3260 * t4459;
    let t13765 = t4415 * t4416 * t13763;
    let t13768 = t3342 * t5424;
    let t13771 = t1248 * t774 * t13671;
    (t13756, t13760, t13763, t13765, t13768, t13771)
}
