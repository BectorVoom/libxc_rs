//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 841/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk841(t8171: f64, t8184: f64, t8716: f64, t8718: f64, t8722: f64, t8742: f64, t8744: f64, t9609: f64, t9611: f64, t9615: f64, t9619: f64, t9623: f64, t9627: f64, t9631: f64, t9634: f64, t9638: f64, t9642: f64, t9646: f64, t9650: f64, t9654: f64) -> f64 {
    let t9911 = 0.32012600194825403606e-1_f64 * t8716 - 0.68598428988911579156e-2_f64 * t8718 - 0.25724410870841842184e-2_f64 * t8722 + 0.37737710747524982482e-2_f64 * t9609 + 0.68598428988911579156e-2_f64 * t9611 - t8171 + t9615 / 16.0_f64 + t9619 / 96.0_f64 - t9623 / 64.0_f64 - t9627 / 192.0_f64 - 0.7640625e-2_f64 * t9631 - 0.42874018118069736972e-3_f64 * t9634 - 0.21437009059034868486e-3_f64 * t9638 + 0.31448092289604152069e-3_f64 * t9642 - 0.62896184579208304138e-3_f64 * t9646 + 0.42874018118069736972e-3_f64 * t9650 - 0.94344276868812456206e-2_f64 * t9654 + 0.916875e-1_f64 * t8742 + 0.61125e-1_f64 * t8744 + t8184;
    t9911
}
