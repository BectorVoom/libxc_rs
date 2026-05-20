//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3095/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3095<F: Float>(t12243: F, t24215: F, t81589: F, t81591: F, t81593: F, t81596: F, t81599: F, t81601: F, t81604: F, t81606: F, t81609: F, t81612: F) -> (F, F) {
    let t81614 = F::cast_from(0.48245938496077605201e2_f64) * t12243 * t24215;
    let t81615 = t81589 + t81591 - t81593 - t81596 + t81599 - t81601 - t81604 - t81606 + t81609 - t81612 + t81614;
    (t81614, t81615)
}
