//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1111/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1111<F: Float>(t2168: F, t6527: F, t2183: F, t6474: F, t13866: F, t1591: F, t128: F, t4145: F, t524: F, t540: F, t114: F, t6358: F, t252: F, t1569: F, t2: F, t386: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20582 = t6527 * t2168;
    let t20589 = t2183 * t6474;
    let t20594 = t1591 * t13866;
    let t20621 = t4145 * t128;
    let t20622 = t524 * t20621;
    let t20623 = t20622 * t540;
    let t20642 = 1.0 / t6358 / t114;
    let t20643 = t20642 * t252;
    let t20646 = t1569 * t1569;
    let t20659 = t1569 * t2 * t386;
    (t20582, t20589, t20594, t20622, t20623, t20642, t20643, t20646, t20659)
}
