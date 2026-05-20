//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1345/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1345<F: Float>(t220: F, t40724: F, t2482: F, t2668: F, t823: F, t159: F, t33127: F, t64: F, t222: F, t124: F, t138: F, t40649: F, t9645: F) -> (F, F, F, F, F) {
    let t40725 = t40724 * t220;
    let t40731 = t2482 * t823 * t2668;
    let t40735 = t64 * t33127 * t159;
    let t40737 = F::new(455.0) / F::new(243.0) * t40735 * t222;
    let t40757 = t138 * t124 * t40649 * t9645;
    (t40725, t40731, t40735, t40737, t40757)
}
