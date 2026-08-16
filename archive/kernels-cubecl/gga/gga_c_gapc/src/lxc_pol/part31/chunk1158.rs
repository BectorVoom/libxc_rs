//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1158/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1158<F: Float>(t128: F, t332: F, t11969: F, t3327: F, t818: F, t7333: F, t875: F, t966: F, t11755: F, t655: F, t761: F, t11960: F, t28920: F, t871: F) -> (F, F, F, F, F) {
    let t33433 = t332 * t128;
    let t33436 = t11969 * t3327 * t33433 * t818;
    let t33441 = t11969 * t7333 * t966 * t128 * t875;
    let t33444 = t761 * t655 * t11755;
    let t33447 = t871 * t11960 * t28920;
    (t33433, t33436, t33441, t33444, t33447)
}
