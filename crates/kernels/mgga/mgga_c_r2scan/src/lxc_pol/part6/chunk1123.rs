//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1123/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1123<F: Float>(t20319: F, t2155: F, t6063: F, t6322: F, t776: F, t1610: F, t2201: F, t5177: F, t2207: F, t5181: F, t19851: F, t546: F, t2135: F, t3433: F, t1569: F, t19965: F) -> (F, F, F, F, F, F, F) {
    let t20324 = t2155 * t6063 * t20319;
    let t20328 = t776 * t6322;
    let t20331 = t2201 * t1610 * t5177;
    let t20334 = t2207 * t1610 * t5181;
    let t20338 = t546 * t19851;
    let t20339 = t3433 * t2135;
    let t20340 = t20338 * t20339;
    let t20342 = t19965 * t1569;
    (t20324, t20328, t20331, t20334, t20338, t20340, t20342)
}
