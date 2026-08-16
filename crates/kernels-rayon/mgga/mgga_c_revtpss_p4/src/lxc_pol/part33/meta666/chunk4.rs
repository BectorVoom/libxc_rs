//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2183/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2183(t22115: f64, t26028: f64, t2018: f64, t22125: f64, t807: f64, t102515: f64, t102526: f64, t102527: f64, t94472: f64, t94474: f64, t94477: f64, t94479: f64, t98194: f64, t98203: f64, t98207: f64) -> f64 {
    let t108583 = t26028 * t22115;
    let t108587 = t807 * t2018 * t22125;
    let t108589 = -t98194 - t94472 + t102515 + t94474 + t98203 - 0.42874018118069736972e-3_f64 * t108583 + t98207 - t94477 + 0.2032800112371413129e-4_f64 * t94479 - t102526 + 0.57165357490759649296e-4_f64 * t108587 - t102527;
    t108589
}
