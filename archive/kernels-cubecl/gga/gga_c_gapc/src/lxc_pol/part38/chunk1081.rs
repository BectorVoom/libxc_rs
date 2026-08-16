//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1081/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1081<F: Float>(t11387: F, t15650: F, t7204: F, t8785: F, t8910: F, t15610: F, t2660: F, t128: F, t332: F, t11969: F, t3327: F, t818: F) -> (F, F, F, F, F) {
    let t33427 = t7204 * t11387 * t15650;
    let t33429 = t8910 * t8785;
    let t33431 = t2660 * t33429 * t15610;
    let t33433 = t332 * t128;
    let t33436 = t11969 * t3327 * t33433 * t818;
    (t33427, t33429, t33431, t33433, t33436)
}
