//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 968/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk968<F: Float>(t1078: F, t11764: F, t3427: F, t3757: F, t277: F, t641: F, t11755: F, t11522: F, t7073: F, t9799: F, t7451: F, t9396: F) -> (F, F, F, F, F, F, F, F) {
    let t11765 = t11764 * t1078;
    let t11767 = t3757 * t3427;
    let t11769 = t277 * t641;
    let t11770 = t11769 * t11755;
    let t11772 = t7073 * t11522;
    let t11773 = t11772 * t9799;
    let t11775 = t7451 * t11522;
    let t11776 = t11775 * t9396;
    (t11765, t11767, t11769, t11770, t11772, t11773, t11775, t11776)
}
