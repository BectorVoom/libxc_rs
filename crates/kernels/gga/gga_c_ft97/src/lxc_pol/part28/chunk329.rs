//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 329/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk329<F: Float>(t184: F, t3658: F, t21: F, t1078: F, t648: F, t1079: F, t363: F, t649: F, t920: F, t18: F, t1577: F, t1528: F) -> (F, F, F, F, F, F) {
    let t3659 = t3658 * t184;
    let t3660 = t3659 * t21;
    let t3663 = t1078 * t648;
    let t3664 = t184 * t21;
    let t3665 = t3663 * t3664;
    let t3668 = t1079 * t363;
    let t3674 = t649 * t920;
    let t3677 = t184 * t18;
    let t3678 = t3677 * t1577;
    let t4406 = t1528 * t920;
    (t3660, t3665, t3668, t3674, t3678, t4406)
}
