//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1036/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1036<F: Float>(t18227: F, t8460: F, t27123: F, t28219: F, t28019: F, t4147: F, t32110: F, t7732: F, t1353: F, t7933: F, t1907: F, t7311: F, t120967: F, t1399: F, t1868: F, t247: F, t561: F) -> (F, F, F, F, F, F, F, F) {
    let t125386 = t18227 * t8460;
    let t125388 = t27123 * t8460;
    let t125390 = t28219 * t8460;
    let t125428 = t4147 * t28019;
    let t125507 = 2.0 * t7732 * t32110;
    let t125559 = t7933 * t1353;
    let t125563 = t1907 * t7311;
    let t125570 = t120967 * t247 * t561 * t1868 * t1399;
    (t125386, t125388, t125390, t125428, t125507, t125559, t125563, t125570)
}
