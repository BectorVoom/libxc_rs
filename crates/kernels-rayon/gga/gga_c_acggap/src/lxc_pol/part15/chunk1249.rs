//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1249/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1249(t32823: f64, t32824: f64, t35635: f64, t35643: f64, t35648: f64, t35653: f64, t35662: f64, t35664: f64, t37636: f64, t37639: f64, t37646: f64, t40083: f64, t40086: f64, t40089: f64, t40092: f64, t40095: f64, t40099: f64, t40101: f64) -> f64 {
    let t41938 = t37636 - t37639 + 0.31448092289604152069e-2_f64 * t35635 - 7.0_f64 / 36.0_f64 * t40083 - 0.4584375e-1_f64 * t40086 - 0.916875e-1_f64 * t40089 - 0.42874018118069736972e-2_f64 * t40092 + 0.21437009059034868486e-2_f64 * t40095 - t32823 + t32824 + 13.0_f64 / 24.0_f64 * t35643 - t37646 - t35648 + t35653 + 0.21437009059034868486e-3_f64 * t40099 - 0.10289764348336736873e0_f64 * t40101 - 0.90035438047946447644e-1_f64 * t35662 - 0.45351183609335988441e-1_f64 * t35664;
    t41938
}
