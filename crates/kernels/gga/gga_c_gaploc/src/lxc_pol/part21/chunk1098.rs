//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1098/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1098<F: Float>(t20157: F, t2085: F, t320: F, t32613: F, t1969: F, t3294: F, t5746: F, t8604: F, t22826: F, t3009: F, t590: F, t7068: F, t23516: F, t32616: F, t22144: F, t32145: F) -> (F, F, F, F, F) {
    let t32860 = 0.27606906686822939768e2 * t320 * t2085 * t20157 * t32613;
    let t32866 = 0.12269736305254639897e2 * t320 * t5746 * t20157 * t8604 * t3294 * t1969;
    let t32870 = 0.30674340763136599742e1 * t22826 * t3009 * t7068 * t590;
    let t32872 = 0.51123901271894332902e1 * t23516 * t32616;
    let t32875 = 0.2044956050875773316e1 * t22144 * t32145;
    (t32860, t32866, t32870, t32872, t32875)
}
