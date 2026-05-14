//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1167/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1167<F: Float>(t33492: F, t33495: F, t33501: F, t33505: F, t33510: F, t33513: F, t33518: F, t33528: F, t33532: F, t33536: F, t33547: F, t33555: F, t33558: F, t33561: F, t33563: F, t33567: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36610 = 0.24581606547037760418e-7 * t33492;
    let t36611 = 0.13340570901084688392e-7 * t33495;
    let t36612 = 0.26194149710963390811e-9 * t33501;
    let t36613 = 0.24581606547037760418e-8 * t33505;
    let t36615 = 0.63350674672043801542e-5 * t33510;
    let t36616 = 0.49520679385353736436e-5 * t33513;
    let t36617 = 0.96681162811134562538e-8 * t33518;
    let t36618 = 0.28198672486580914074e-8 * t33528;
    let t36619 = 0.57920616843011475696e-5 * t33532;
    let t36621 = 0.44197102999375800017e-7 * t33536;
    let t36623 = 0.50083268227528753081e-5 * t33547;
    let t36625 = 0.6070699179094394313e-6 * t33555;
    let t36626 = 0.10793703140429833089e-5 * t33558;
    let t36627 = 0.64085799349094910026e-6 * t33561;
    let t36628 = 0.64085799349094910026e-6 * t33563;
    let t36630 = 0.54924190264999682021e-4 * t33567;
    (t36610, t36611, t36612, t36613, t36615, t36616, t36617, t36618, t36619, t36621, t36623, t36625, t36626, t36627, t36628, t36630)
}
