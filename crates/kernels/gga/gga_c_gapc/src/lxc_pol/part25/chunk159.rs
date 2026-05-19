//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 159/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk159<F: Float>(t10: F, t103: F, t160: F, t161: F, t164: F, t421: F, t540: F, t544: F, t547: F, t551: F, t99: F, t115: F) -> (F, F) {
    let t560 = F::new(0.619125e-2) * t540 * t161 - F::new(0.123825e-1) * t544 * t547 - F::new(0.619125e-2) * t160 * t551 - F::cast_from(0.53062222222222222221e-1_f64) * t103 * t10 * t99 - F::cast_from(0.79593333333333333331e-1_f64) * t103 * t164 * t421;
    let t561 = t560 * t115;
    (t560, t561)
}
