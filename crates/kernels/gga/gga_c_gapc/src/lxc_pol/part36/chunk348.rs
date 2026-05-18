//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 348/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk348<F: Float>(t152: F, t1552: F, t172: F, t129: F, t19: F, t464: F, t1412: F, t188: F, t20: F, t128: F, t173: F, t432: F, t640: F) -> (F, F, F, F, F, F, F) {
    let t1553 = t1552 * t152;
    let t1554 = t1553 * t172;
    let t1555 = t129 * t1554;
    let t1558 = t464 * t19;
    let t1559 = t1412 * t1558;
    let t1560 = t20 * t188;
    let t1561 = t1560 * t128;
    let t1562 = t1561 * t173;
    let t1565 = t432 * t640;
    (t1554, t1555, t1559, t1560, t1561, t1562, t1565)
}
