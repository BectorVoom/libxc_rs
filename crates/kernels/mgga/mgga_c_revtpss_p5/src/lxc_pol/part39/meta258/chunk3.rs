//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 962/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk962<F: Float>(t114: F, t658: F, t8268: F, t69: F, t8257: F, t8258: F, t8260: F, t8264: F, t8267: F) -> (F, F) {
    let t115 = F::new(1.0) < t114;
    let t8269 = t8268 * t658;
    let t8273 = piecewise3::<F>(t115, F::new(0.0), t8257 + t8258 * t8260 / F::new(4.0) + F::new(5.0) / F::new(24.0) * t69 * t8264 - F::new(5.0) / F::new(24.0) * t8267 * t8269);
    (t8269, t8273)
}
