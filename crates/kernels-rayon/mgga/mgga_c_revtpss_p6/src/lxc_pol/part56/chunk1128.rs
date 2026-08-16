//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1128/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1128(t126078: f64, t2747: f64, t31767: f64, t31772: f64, t124: f64, t1579: f64, t800: f64, t815: f64, t32469: f64, t32474: f64, t119767: f64, t1544: f64, t247: f64, t257: f64, t837: f64) -> (f64, f64, f64, f64) {
    let t126158 = t31767 * t2747 * t31772 * t126078;
    let t126163 = t815 * t800 * t124 * t1579;
    let t126164 = t32469 * t126163;
    let t126166 = t32474 * t126163;
    let t126182 = t119767 * t247 * t257 * t1544 * t837;
    (t126158, t126164, t126166, t126182)
}
