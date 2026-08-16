//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1236/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1236(t30926: f64, t32664: f64, t35123: f64, t35125: f64, t35160: f64, t35162: f64, t35163: f64, t35164: f64, t35167: f64, t37386: f64, t37408: f64, t37409: f64, t39720: f64, t39724: f64, t39733: f64, t39735: f64, t39737: f64) -> f64 {
    let t41771 = t37386 - 0.18868855373762491241e-1_f64 * t39720 + 0.21437009059034868486e-2_f64 * t39724 + t35123 - 0.11321313224257494745e-1_f64 * t30926 - t35125 - t37408 - t37409 + t35160 - t35162 - 0.4584375e-1_f64 * t39733 - t39735 / 4.0_f64 - t39737 / 6.0_f64 + t35163 - t35164 - t32664 - t35167;
    t41771
}
