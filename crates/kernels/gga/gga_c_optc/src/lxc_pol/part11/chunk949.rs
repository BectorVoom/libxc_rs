//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 949/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk949<F: Float>(t102: F, t108: F, t176: F, t203: F, t23331: F, t1974: F, t6569: F, t732: F, t188: F, t202: F, t23047: F, t6602: F, t740: F, t195: F, t6: F, t8285: F, t92: F, t93: F) -> (F, F, F, F, F, F, F) {
    let t23336 = t176 * t23331 * t102 * t108 * t203 / 2.0;
    let t23392 = t1974 * t1974;
    let t23393 = 1.0 / t23392;
    let t23413 = 1820.0 / 27.0 * t732 * t6569;
    let t23431 = 7280.0 / 81.0 * t188 * t23047 * t202;
    let t23438 = 14.0 / 3.0 * t6602 * t740;
    let t23471 = t6 * t195;
    let t23518 = 1.0 / t8285 / t92 * t93;
    (t23336, t23393, t23413, t23431, t23438, t23471, t23518)
}
