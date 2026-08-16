//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 840/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk840(t8146: f64, t8619: f64, t8625: f64, t8650: f64, t8658: f64, t8666: f64, t8680: f64, t8682: f64, t8684: f64, t8690: f64, t8694: f64, t8706: f64, t8710: f64, t8712: f64, t8714: f64, t9584: f64, t9590: f64, t9594: f64, t9598: f64, t9602: f64) -> f64 {
    let t9892 = t8146 + 0.5603125e-1_f64 * t8619 + 7.0_f64 / 72.0_f64 * t8625 + 0.12579236915841660828e-2_f64 * t9584 + 0.21437009059034868486e-2_f64 * t9590 + 0.85748036236139473944e-3_f64 * t9594 - 0.31448092289604152068e-2_f64 * t9598 - 0.18868855373762491241e-2_f64 * t9602 + 0.21437009059034868486e-2_f64 * t8650 - 0.18868855373762491241e-2_f64 * t8658 + 0.41930789719472202758e-3_f64 * t8666 + 11.0_f64 / 96.0_f64 * t8680 + 11.0_f64 / 288.0_f64 * t8682 + 7.0_f64 / 36.0_f64 * t8684 - 0.42874018118069736972e-3_f64 * t8690 + 0.34299214494455789578e-2_f64 * t8694 - 0.34299214494455789578e-2_f64 * t8706 + 0.68598428988911579156e-2_f64 * t8710 + 0.16006300097412701803e-1_f64 * t8712 - 0.16006300097412701803e-1_f64 * t8714;
    t9892
}
