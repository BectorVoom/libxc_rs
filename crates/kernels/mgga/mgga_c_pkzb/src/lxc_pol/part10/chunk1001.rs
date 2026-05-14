//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1001/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1001<F: Float>(t2243: F, t8219: F, t7930: F, t6090: F, t6093: F, t6348: F, t7947: F, t7955: F, t6088: F, t352: F, t2259: F, t3088: F, t365: F, t6272: F, t8191: F, t8194: F, t8197: F, t8201: F, t8204: F, t8208: F, t8211: F, t8216: F, t8218: F) -> (F, F, F, F, F, F, F) {
    let t8221 = 0.16081979498692535067e2 * t8219 * t2243;
    let t8225 = 0.34246666666666666666e-1 * t7930;
    let t8227 = -t6348 + 0.45662222222222222222e-1 * t6090 - 0.17123333333333333333e-1 * t6093 + 0.22831111111111111111e-1 * t7955 - t8225 + 0.5137e-1 * t7947;
    let t8233 = 0.35616666666666666666e-1 * t7930;
    let t8235 = -t6088 + 0.47488888888888888888e-1 * t6090 - 0.17808333333333333333e-1 * t6093 + 0.23744444444444444444e-1 * t7955 - t8233 + 0.53425e-1 * t7947;
    let t8237 = 0.621814e-1 * t8235 * t352;
    let t8238 = t8191 + t8194 + t8197 - t8201 - t8204 - t8208 - 4.0 * t6272 * t3088 - 2.0 * t8211 * t2259 - t8216 - t8218 - t8221 - 0.310907e-1 * t8227 * t365 + t8237;
    (t8221, t8225, t8227, t8233, t8235, t8237, t8238)
}
