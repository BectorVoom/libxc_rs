//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1060/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1060<F: Float>(t146: F, t2002: F, t2111: F, t2116: F, t108: F, t6990: F, t110: F, t2164: F, t7012: F, t22154: F, t56: F, t148: F, t151: F) -> (F, F, F, F, F) {
    let t23027 = t146 * t2111 * t2002;
    let t23028 = t23027 * t2116;
    let t23038 = t6990 * t108;
    let t23040 = t146 * t23038 * t110;
    let t23045 = t2164 * t7012;
    let t23047 = t22154 * t56;
    let t23050 = F::cast_from(0.15626226085348680785e2_f64) * t148 * t23047 * t151;
    (t23028, t23040, t23045, t23047, t23050)
}
