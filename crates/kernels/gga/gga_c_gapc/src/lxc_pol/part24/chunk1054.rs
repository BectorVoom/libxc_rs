//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1054/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1054<F: Float>(t11401: F, t3074: F, t35085: F, t27036: F, t27043: F, t35139: F, t11408: F, t561: F, t8951: F, t11413: F, t8960: F, t19546: F, t33623: F, t5462: F, t11224: F, t518: F) -> (F, F, F, F, F, F) {
    let t35348 = t3074 * t11401;
    let t35349 = t35085 * t35348;
    let t35352 = t27036 * t35139 * t27043;
    let t35355 = t561 * t11408 * t8951;
    let t35358 = t561 * t11413 * t8960;
    let t35361 = t5462 * t33623 * t19546;
    let t35379 = t518 * t11224;
    (t35349, t35352, t35355, t35358, t35361, t35379)
}
