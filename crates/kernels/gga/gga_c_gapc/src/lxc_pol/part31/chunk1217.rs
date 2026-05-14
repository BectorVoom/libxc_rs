//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1217/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1217<F: Float>(t33834: F, t33838: F, t36723: F, t36725: F, t36727: F, t36728: F, t36729: F, t36730: F, t36731: F, t36732: F, t36733: F, t33893: F, t33920: F, t36749: F, t36750: F, t36751: F, t36752: F, t36753: F, t36754: F, t36755: F, t36756: F, t36758: F) -> (F, F) {
    let t38767 = -t36723 + 0.2445773654513888889e-4 * t33834 - t36725 - 0.18115908419564701086e-6 * t33838 + t36727 - t36728 + t36729 - t36730 + t36731 + t36732 - t36733;
    let t38774 = 0.24598298249421296296e-6 * t33893 - t36749 - t36750 + t36751 - t36752 + t36753 - t36754 + t36755 - t36756 + 0.50595483470764842602e-7 * t33920 + t36758;
    (t38767, t38774)
}
