//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 890/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk890<F: Float>(t2034: F, t33228: F, t1990: F, t46833: F, t10043: F, t1979: F, t1982: F, t458: F, t1971: F, t236: F, t38454: F, t6096: F, t46736: F, t739: F, t16503: F, t3369: F, t571: F, t8430: F) -> (F, F, F, F, F, F) {
    let t47295 = t33228 * t2034;
    let t47302 = t46833 * t1990;
    let t47306 = t10043 * t458 * t1979 * t1982;
    let t47310 = t38454 * t1971 * t236 * t6096;
    let t47316 = t739 * t46736;
    let t47321 = t16503 * t3369 * t571 * t8430;
    (t47295, t47302, t47306, t47310, t47316, t47321)
}
