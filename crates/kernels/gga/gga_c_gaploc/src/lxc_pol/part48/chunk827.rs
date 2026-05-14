//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 827/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk827<F: Float>(t11408: F, t1445: F, t1562: F, t2293: F, t10252: F, t10615: F, t900: F, t20884: F, t37667: F, t13397: F, t587: F, t589: F, t44295: F, t4820: F, t6824: F, t13387: F, t4379: F) -> (F, F, F, F, F, F) {
    let t46233 = 0.69017266717057349418e1 * t1562 * t1445 * t11408 * t2293;
    let t46235 = t10615 * t900 * t10252;
    let t46237 = t37667 * t20884;
    let t46240 = t587 * t589 * t13397;
    let t46244 = 0.79445533226334281487e-1 * t6824 * t4820 * t44295;
    let t46245 = t4379 * t13387;
    (t46233, t46235, t46237, t46240, t46244, t46245)
}
