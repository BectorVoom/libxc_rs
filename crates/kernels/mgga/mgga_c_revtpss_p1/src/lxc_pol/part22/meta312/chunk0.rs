//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1750/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1750<F: Float>(t58: F, t59: F, t10199: F, t2851: F, t78: F, t3361: F, t81: F, t116: F, t2319: F) -> (F, F, F, F, F) {
    let t10368 = F::new(1.0) / t59 / t58;
    let t10379 = F::new(1232.0) / F::new(27.0) * t10199;
    let t10389 = F::new(1.0) / t78 / t2851;
    let t10398 = F::new(1.0) / t81 / t3361;
    let t10416 = t2319 * t116;
    (t10368, t10379, t10389, t10398, t10416)
}
