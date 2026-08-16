//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 470/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk470<F: Float>(t2619: F, t612: F, t891: F, t918: F, t617: F, t1695: F, t933: F, t327: F, t442: F, t6: F, t786: F, t1087: F) -> (F, F, F, F, F, F, F) {
    let t2620 = t2619 * t612;
    let t2621 = t918 * t891;
    let t2622 = t617 * t2621;
    let t2625 = t933 * t1695;
    let t2626 = t442 * t327;
    let t2627 = t786 * t6;
    let t2628 = t1087 * t2627;
    (t2620, t2621, t2622, t2625, t2626, t2627, t2628)
}
