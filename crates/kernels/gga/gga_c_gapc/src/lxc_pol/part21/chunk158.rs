//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 158/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk158<F: Float>(t14: F, t543: F, t1: F, t108: F, t3: F, t78: F, t106: F, t70: F, t402: F, t10: F, t103: F, t160: F, t161: F, t164: F, t421: F, t540: F, t99: F) -> (F, F, F, F, F, F, F) {
    let t544 = t543 * t14;
    let t545 = t108 * t1;
    let t546 = t3 * t78;
    let t547 = t545 * t546;
    let t550 = t106 * t70;
    let t551 = t550 * t402;
    let t560 = 0.619125e-2 * t540 * t161 - 0.123825e-1 * t544 * t547 - 0.619125e-2 * t160 * t551 - 0.53062222222222222221e-1 * t103 * t10 * t99 - 0.79593333333333333331e-1 * t103 * t164 * t421;
    (t544, t545, t546, t547, t550, t551, t560)
}
