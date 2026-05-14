//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1212/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1212<F: Float>(t33464: F, t33474: F, t36596: F, t36597: F, t36599: F, t36600: F, t36601: F, t36602: F, t36604: F, t36605: F, t36606: F, t33507: F, t36609: F, t36610: F, t36611: F, t36612: F, t36613: F, t36615: F, t36616: F, t36617: F, t36618: F, t36619: F) -> (F, F) {
    let t38740 = -t36596 - t36597 - 0.18115908419564701086e-6 * t33464 + t36599 - t36600 + t36601 + t36602 - 0.56912804804009946682e-7 * t33474 - t36604 + t36605 + t36606;
    let t38743 = -t36609 - t36610 + t36611 - t36612 + t36613 + 0.67632724766374884054e-5 * t33507 - t36615 - t36616 + t36617 + t36618 - t36619;
    (t38740, t38743)
}
