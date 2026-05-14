//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1000/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1000<F: Float>(t2648: F, t3769: F, t11834: F, t16403: F, t7191: F, t1026: F, t2674: F, t9827: F, t11902: F, t19161: F, t18317: F, t33148: F, t7259: F, t11974: F, t3285: F, t3289: F) -> (F, F, F, F, F, F, F) {
    let t33343 = t3769 * t2648;
    let t33346 = t11834 * t16403 * t7191;
    let t33349 = t2674 * t1026 * t9827;
    let t33353 = t11902 * t19161;
    let t33356 = t7259 * t33148 * t18317;
    let t33358 = t11974 * t3285;
    let t33360 = t11974 * t3289;
    (t33343, t33346, t33349, t33353, t33356, t33358, t33360)
}
