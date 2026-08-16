//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 942/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk942<F: Float>(t2056: F, t7600: F, t2074: F, t30456: F, t1980: F, t31107: F, t7458: F, t1998: F, t3292: F, t1017: F, t1992: F, t2095: F) -> (F, F, F, F, F) {
    let t31477 = t7600 * t2056;
    let t31479 = t30456 * t2074;
    let t31482 = t1980 * t7458 * t31107;
    let t31484 = t1998 * t3292;
    let t31487 = t2095 * t1992 * t1017;
    (t31477, t31479, t31482, t31484, t31487)
}
