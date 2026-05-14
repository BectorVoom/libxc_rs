//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 418/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk418<F: Float>(t165: F, t6616: F, t28: F, t1058: F, t1360: F, t5855: F, t925: F, t2221: F, t1017: F, t1359: F) -> (F, F, F, F, F, F, F) {
    let t6617 = t6616 * t165;
    let t6618 = t28 * t6617;
    let t6621 = t1360 * t1058;
    let t6622 = t28 * t6621;
    let t6626 = t5855 * t925;
    let t6627 = t2221 * t6626;
    let t6630 = t1359 * t1017;
    (t6617, t6618, t6621, t6622, t6626, t6627, t6630)
}
