//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 987/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk987<F: Float>(t1992: F, t550: F, t6976: F, t84441: F, t22704: F, t22705: F, t31627: F, t1351: F, t7191: F, t31632: F, t6883: F, t22724: F, t31623: F) -> (F, F, F, F, F) {
    let t115420 = t1992 * t6976 * t84441 * t550;
    let t115423 = t22704 * t22705 * t31627;
    let t115428 = t1992 * t6976 * t7191 * t1351 * t550;
    let t115430 = t6883 * t31632;
    let t115432 = t22724 * t31623;
    (t115420, t115423, t115428, t115430, t115432)
}
