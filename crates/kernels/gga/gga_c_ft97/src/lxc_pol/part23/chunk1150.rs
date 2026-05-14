//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1150/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1150<F: Float>(t2399: F, t6909: F, t89: F, t1882: F, t28239: F, t28422: F, t28334: F, t28269: F, t1456: F, t9895: F, t255: F, t6074: F, t7514: F, t53798: F, t6161: F, t6942: F, t8232: F) -> (F, F, F, F, F, F, F, F, F) {
    let t111290 = t89 * t2399 * t6909;
    let t111310 = 4.0 / 9.0 * t1882 * t28239;
    let t111320 = 4.0 / 9.0 * t1882 * t28422;
    let t111322 = 2.0 / 9.0 * t1882 * t28334;
    let t111324 = 2.0 / 9.0 * t1882 * t28269;
    let t111330 = t9895 * t1456;
    let t111356 = t7514 * t255 * t6074;
    let t111363 = t53798 * t6161;
    let t111389 = t8232 * t6942;
    (t111290, t111310, t111320, t111322, t111324, t111330, t111356, t111363, t111389)
}
