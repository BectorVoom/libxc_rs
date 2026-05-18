//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1315/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1315<F: Float>(t187: F, t97635: F, t97637: F, t97638: F, t97641: F, t97643: F, t97645: F, t97647: F, t97650: F, t97652: F, t97654: F, t97657: F, t97845: F, t97852: F, t97854: F, t97856: F, t97862: F, t97870: F, t97875: F, t97880: F, t98957: F, t99733: F) -> F {
    let t99737 = t97635 + t97637 - t97638 + t97641 + t97643 + t97645 - t97647 - t97650 - t97652 + t97654 + t97657 + t187 * (t97845 + t97880 + t98957 + t99733) - t97852 - t97854 + t97856 + t97862 + t97870 + t97875;
    t99737
}
