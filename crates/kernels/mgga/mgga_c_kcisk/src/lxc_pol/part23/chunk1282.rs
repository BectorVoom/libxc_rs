//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1282/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1282<F: Float>(t32022: F, t32176: F, t32173: F, t1328: F, t13830: F, t1308: F, t388: F, t41006: F, t20233: F, t32009: F, t32008: F, t32105: F, t9422: F, t9434: F, t32069: F, t3936: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110548 = t32022 * t32176;
    let t110556 = t32022 * t32173;
    let t110558 = t13830 * t1328;
    let t110566 = t41006 * t388 * t1308;
    let t110577 = t20233 * t32009;
    let t110578 = t32008 * t110577;
    let t110593 = t9422 * t32105;
    let t110595 = t9434 * t32105;
    let t110605 = t3936 * t32069;
    (t110548, t110556, t110558, t110566, t110577, t110578, t110593, t110595, t110605)
}
