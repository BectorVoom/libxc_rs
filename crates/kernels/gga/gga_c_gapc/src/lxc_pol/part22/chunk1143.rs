//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1143/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1143<F: Float>(t11902: F, t19161: F, t18317: F, t33148: F, t7259: F, t11974: F, t3285: F, t3289: F, t2572: F, t33328: F, t11397: F, t932: F) -> (F, F, F, F, F) {
    let t33353 = t11902 * t19161;
    let t33356 = t7259 * t33148 * t18317;
    let t33358 = t11974 * t3285;
    let t33360 = t11974 * t3289;
    let t33363 = t33328 * t2572;
    let t33364 = t932 * t11397 * t33363;
    (t33353, t33356, t33358, t33360, t33364)
}
