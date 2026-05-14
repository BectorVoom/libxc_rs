//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1269/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1269<F: Float>(t28738: F, t28742: F, t33616: F, t33619: F, t33624: F, t33626: F, t33630: F, t33633: F, t33637: F, t33640: F, t33642: F, t33645: F, t33649: F, t33651: F, t33653: F, t33656: F) -> (F,) {
    let t39249 = t33616 - t33619 - t33624 - t33626 - t33630 - 0.15337170381568299871e1 * t28738 - 0.15337170381568299871e1 * t28742 + t33633 - t33637 - t33640 - t33642 - t33645 - t33649 - t33651 - t33653 + t33656;
    (t39249,)
}
