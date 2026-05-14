//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 685/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk685<F: Float>(t7523: F, t2516: F, t808: F, t243: F, t251: F, t2519: F, t256: F, t7341: F, t224: F, t2269: F) -> (F, F, F, F, F, F) {
    let t7787 = 0.16068111111111111111e1 * t7523;
    let t7798 = 1.0 / t2516 / t808;
    let t7799 = t243 * t7798;
    let t7801 = 1.0 / t2519 / t251;
    let t7813 = t256 * t7341;
    let t7856 = 1.0 / t224 / t2269;
    (t7787, t7798, t7799, t7801, t7813, t7856)
}
