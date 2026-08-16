//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1485/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1485(t10199: f64, t2339: f64, t116: f64, t31292: f64, t1913: f64, t8302: f64, t2192: f64, t5789: f64, t2184: f64, t5808: f64, t31328: f64, t575: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t117544 = t10199 * t2339;
    let t117758 = t116 * t31292;
    let t117772 = 2.0_f64 * t1913 * t8302;
    let t117774 = 2.0_f64 * t5789 * t2192;
    let t117781 = 2.0_f64 * t2184 * t5808;
    let t117783 = 2.0_f64 * t31328 * t575;
    (t117544, t117758, t117772, t117774, t117781, t117783)
}
