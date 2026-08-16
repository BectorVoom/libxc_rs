//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1199/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1199(t2167: f64, t8249: f64, t1913: f64, t8978: f64, t35034: f64, t571: f64, t127442: f64, t127447: f64, t127449: f64, t127453: f64, t127455: f64, t127459: f64, t127462: f64, t127465: f64, t127468: f64, t127472: f64, t127480: f64, t129541: f64, t129543: f64, t132119: f64, t1918: f64, t32373: f64, t32377: f64, t33565: f64, t573: f64, param_d: f64) -> (f64, f64, f64, f64) {
    let t132135 = t2167 * t8249;
    let t132141 = t1913 * t8978;
    let t132144 = t571 * t35034;
    let t132152 = t132119 * t573 * param_d + 3.0_f64 * t1918 * t33565 + t127442 + t127447 + t127449 + t127453 + t127455 + t127459 + t127462 + t127465 + t127468 + t127472 + t127480 + 6.0_f64 * t129541 + 12.0_f64 * t129543 + t32373 + t32377;
    (t132135, t132141, t132144, t132152)
}
