//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 718/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk718<F: Float>(t119: F, t492: F, t2886: F, t118: F, t1845: F, t61: F, t2881: F, t2921: F, t8350: F, t2925: F, t1457: F, t424: F) -> (F, F, F, F, F) {
    let t8482 = t492 * t119;
    let t8483 = t8482 * t2886;
    let t8485 = t1845 * t118;
    let t8486 = t61 * t8485;
    let t8487 = t8486 * t2881;
    let t8489 = t8350 * t2921;
    let t8490 = t8489 * t2925;
    let t8492 = t424 * t1457;
    (t8483, t8487, t8489, t8490, t8492)
}
