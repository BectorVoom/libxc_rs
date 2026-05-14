//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 605/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk605<F: Float>(t3751: F, t3752: F, t122: F, t825: F, t125: F, t311: F) -> (F, F, F, F) {
    let t3753 = t3751 * t3752;
    let t3755 = t825 * t122;
    let t3756 = t3755 * t125;
    let t3757 = t311 * t3756;
    (t3753, t3755, t3756, t3757)
}
