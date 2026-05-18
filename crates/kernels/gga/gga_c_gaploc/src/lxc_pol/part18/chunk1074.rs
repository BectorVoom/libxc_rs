//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1074/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1074<F: Float>(t24586: F, t7290: F, t24350: F, t1980: F, t8774: F, t2154: F, t2936: F, t6134: F, t8792: F, t1: F, t106: F, t316: F, t8720: F) -> (F, F, F, F, F, F) {
    let t24741 = t7290 * t24586;
    let t24745 = t7290 * t24350;
    let t24751 = t1980 * t8774;
    let t24777 = t2154 * t2936;
    let t24784 = t6134 * t8792;
    let t24817 = t8720 * t1 * t106 * t316;
    (t24741, t24745, t24751, t24777, t24784, t24817)
}
