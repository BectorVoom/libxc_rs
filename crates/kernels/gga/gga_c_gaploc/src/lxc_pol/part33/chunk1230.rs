//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1230/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1230<F: Float>(t1969: F, t20157: F, t320: F, t3294: F, t5746: F, t8604: F, t22826: F, t3009: F, t590: F, t7068: F, t23516: F, t32616: F) -> (F, F, F) {
    let t32866 = F::new(0.12269736305254639897e2) * t320 * t5746 * t20157 * t8604 * t3294 * t1969;
    let t32870 = F::new(0.30674340763136599742e1) * t22826 * t3009 * t7068 * t590;
    let t32872 = F::new(0.51123901271894332902e1) * t23516 * t32616;
    (t32866, t32870, t32872)
}
