//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 322/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk322<F: Float>(t1416: F, t152: F, t172: F, t19: F, t20: F, t435: F, t505: F) -> (F, F, F, F, F) {
    let t1417 = t1416 * t152;
    let t1418 = t172 * t19;
    let t1419 = t1418 * t20;
    let t1420 = t1417 * t1419;
    let t1423 = t435 * t505;
    (t1417, t1418, t1419, t1420, t1423)
}
