//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1006/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1006<F: Float>(t1426: F, t1882: F, t121011: F, t247: F, t94396: F, t125627: F, t1399: F, t1892: F, t31805: F, t1381: F, t8590: F, t121181: F, t5741: F, t121146: F, t32195: F, t32206: F, t3936: F, t5591: F) -> (F, F, F, F, F, F) {
    let t125639 = t1426 * t1882;
    let t125642 = t121011 * t247 * t125639 * t94396;
    let t125646 = t121011 * t247 * t125627 * t1399;
    let t125648 = t31805 * t1892;
    let t125650 = t125648 * t8590 * t1381;
    let t125652 = t121181 * t5741;
    let t125654 = t121146 * t5741;
    let t125659 = t32206 * t3936 * t32195 * t5591;
    (t125642, t125646, t125650, t125652, t125654, t125659)
}
