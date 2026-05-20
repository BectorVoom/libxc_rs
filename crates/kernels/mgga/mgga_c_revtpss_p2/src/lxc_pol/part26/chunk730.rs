//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 730/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk730<F: Float>(t730: F, t9446: F, t2596: F, t675: F, t215: F, t723: F, t2553: F, t738: F, t2491: F, t177: F, t9417: F, t2495: F, t9368: F) -> (F, F, F, F, F, F, F, F) {
    let t9447 = t9446 * t730;
    let t9450 = t675 * t2596;
    let t9454 = t215 * t723;
    let t9461 = t675 * t2553;
    let t9469 = t215 * t738;
    let t9476 = t675 * t2491;
    let t9480 = t177 * t9417;
    let t9481 = t9368 * t2495;
    (t9447, t9450, t9454, t9461, t9469, t9476, t9480, t9481)
}
