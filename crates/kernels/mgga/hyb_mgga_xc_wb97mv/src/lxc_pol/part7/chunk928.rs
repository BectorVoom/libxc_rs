//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 928/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk928<F: Float>(t3296: F, t676: F, t3304: F, t549: F, t136: F, t1234: F, t1286: F, t2015: F, t2165: F, t3167: F, t3291: F, t683: F, t686: F, t8595: F, t8601: F, t8604: F, t8607: F, t8609: F, t8617: F, t8620: F) -> (F, F, F, F) {
    let t8623 = t676 * t3296 / 32.0;
    let t8626 = t549 * t3304;
    let t8628 = t136 * t8626 / 32.0;
    let t8629 = t683 * t3167 * t8595 / 16.0 - t8601 - t8604 - 7.0 / 96.0 * t8607 - t683 * t686 * t8609 / 64.0 - 3.0 / 64.0 * t2015 * t1286 - 3.0 / 32.0 * t676 * t3291 - 7.0 / 32.0 * t8617 + t8620 / 96.0 - t8623 - 3.0 / 64.0 * t1234 * t2165 - t8628;
    (t8623, t8626, t8628, t8629)
}
