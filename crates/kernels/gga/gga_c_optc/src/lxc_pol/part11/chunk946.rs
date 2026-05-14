//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 946/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk946<F: Float>(t108: F, t6990: F, t110: F, t146: F, t22154: F, t56: F, t148: F, t151: F, t6567: F, t671: F, t155: F, t2078: F, t2157: F, t652: F, t6991: F, t156: F, t2155: F) -> (F, F, F, F, F, F, F) {
    let t23038 = t6990 * t108;
    let t23040 = t146 * t23038 * t110;
    let t23047 = t22154 * t56;
    let t23050 = 0.15626226085348680785e2 * t148 * t23047 * t151;
    let t23065 = t146 * t671 * t6567;
    let t23071 = t155 * t2157 * t2078;
    let t23077 = t155 * t6991 * t652;
    let t23095 = 1.0 / t2155 / t156;
    (t23040, t23047, t23050, t23065, t23071, t23077, t23095)
}
