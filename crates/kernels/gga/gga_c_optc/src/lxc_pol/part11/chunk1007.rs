//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1007/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1007<F: Float>(t1790: F, t1792: F, t533: F, t6446: F, t1758: F, t1784: F, t6452: F, t6454: F, t209: F, t60: F, t6472: F, t6519: F) -> (F, F, F, F) {
    let t22281 = F::new(0.64327297288604419288e2) * t1790 * t6446 * t1792 * t533;
    let t22285 = F::new(0.3103500882342370105e4) * t6452 * t1758 * t6454 * t1784;
    let t22287 = t60 * t209;
    let t22290 = F::new(0.13012297059337829057e0) * t22287 * t6519 * t6472;
    (t22281, t22285, t22287, t22290)
}
