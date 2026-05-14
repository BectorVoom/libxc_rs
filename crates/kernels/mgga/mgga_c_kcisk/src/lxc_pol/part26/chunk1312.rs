//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1312/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1312<F: Float>(t1411: F, t1440: F, t32045: F, t8251: F, t1339: F, t26860: F, t26958: F, t9461: F, t26901: F, t26778: F, t33608: F, t110605: F, t2075: F, t33398: F, t113671: F, t2173: F, t33358: F, t442: F) -> (F, F, F, F, F, F, F) {
    let t118866 = t1411 * t32045 * t8251 * t1440;
    let t118869 = t1339 * t32045 * t26860;
    let t118872 = t1339 * t9461 * t26958;
    let t118875 = t1339 * t9461 * t26901;
    let t118878 = t1339 * t33608 * t26778;
    let t118882 = t110605 * t2075 * t33398;
    let t118891 = t113671 * t2173 * t442 * t33358;
    (t118866, t118869, t118872, t118875, t118878, t118882, t118891)
}
