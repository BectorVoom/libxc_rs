//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1236/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1236<F: Float>(t30926: F, t32664: F, t35123: F, t35125: F, t35160: F, t35162: F, t35163: F, t35164: F, t35167: F, t37386: F, t37408: F, t37409: F, t39720: F, t39724: F, t39733: F, t39735: F, t39737: F) -> F {
    let t41771 = t37386 - F::new(0.18868855373762491241e-1) * t39720 + F::new(0.21437009059034868486e-2) * t39724 + t35123 - F::new(0.11321313224257494745e-1) * t30926 - t35125 - t37408 - t37409 + t35160 - t35162 - F::new(0.4584375e-1) * t39733 - t39735 / F::new(4.0) - t39737 / F::new(6.0) + t35163 - t35164 - t32664 - t35167;
    t41771
}
