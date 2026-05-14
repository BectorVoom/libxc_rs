//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 940/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk940<F: Float>(t1569: F, t24118: F, t1553: F, t938: F, t113: F, t7204: F, t6363: F, t920: F, t481: F, t7469: F, t2568: F, t3433: F, t2563: F, t1550: F, t7338: F, t2252: F, t921: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24161 = t24118 * t1569;
    let t24165 = t938 * t1553;
    let t24166 = t24165 * t1569;
    let t24172 = t7204 * t113;
    let t24209 = t920 * t6363;
    let t24454 = t7469 * t481;
    let t24521 = t3433 * t2568;
    let t24573 = t3433 * t2563;
    let t24714 = t7338 * t1550;
    let t24750 = t921 * t2252;
    (t24161, t24165, t24166, t24172, t24209, t24454, t24521, t24573, t24714, t24750)
}
