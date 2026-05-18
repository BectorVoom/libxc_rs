//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 524/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk524<F: Float>(t2679: F, t2685: F, t2684: F, t1589: F, t948: F, t1628: F, t965: F, t2586: F, t531: F, t2530: F, t808: F, t568: F) -> (F, F, F, F, F, F, F) {
    let t2686 = t2685 * t2679;
    let t2687 = t2684 * t2686;
    let t2689 = t1589 * t948;
    let t2692 = t1628 * t965;
    let t2699 = t531 * t2586;
    let t2704 = t808 * t2530;
    let t2705 = t568 * t2704;
    (t2686, t2687, t2689, t2692, t2699, t2704, t2705)
}
