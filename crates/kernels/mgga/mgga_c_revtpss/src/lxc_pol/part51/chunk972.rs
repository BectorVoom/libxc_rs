//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 972/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk972<F: Float>(t1907: F, t7311: F, t28196: F, t28197: F, t120967: F, t1399: F, t1868: F, t247: F, t561: F, t120962: F, t32284: F, t5705: F, t5696: F, t120952: F, t1885: F, t5661: F) -> (F, F, F, F, F, F) {
    let t125563 = t1907 * t7311;
    let t125566 = 4.0 * t28196 * t28197 * t125563;
    let t125570 = t120967 * t247 * t561 * t1868 * t1399;
    let t125573 = t32284 * t120962 * t5705;
    let t125576 = t32284 * t120962 * t5696;
    let t125578 = t120952 * t1885;
    let t125580 = t32284 * t5661;
    (t125566, t125570, t125573, t125576, t125578, t125580)
}
