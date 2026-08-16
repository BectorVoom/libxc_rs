//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 966/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk966<F: Float>(t3284: F, t7200: F, t11741: F, t11387: F, t2580: F, t7204: F, t11483: F, t933: F, t2597: F, t7735: F, t277: F, t655: F) -> (F, F, F, F, F, F, F, F) {
    let t11742 = t3284 * t7200;
    let t11743 = t11741 * t11742;
    let t11745 = t11387 * t2580;
    let t11746 = t7204 * t11745;
    let t11748 = t933 * t11483;
    let t11749 = t2597 * t7735;
    let t11750 = t11748 * t11749;
    let t11752 = t277 * t655;
    (t11742, t11743, t11745, t11746, t11748, t11749, t11750, t11752)
}
