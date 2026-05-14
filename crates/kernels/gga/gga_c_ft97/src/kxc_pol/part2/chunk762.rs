//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 762/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk762<F: Float>(t13542: F, t11176: F, t3747: F, t13315: F, t9568: F, t92: F, t13352: F, t2404: F, t13320: F, t3051: F, t13309: F, t13346: F, t683: F, t13301: F, t13296: F, t665: F, t668: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13543 = 4.0 / 9.0 * t13542;
    let t13544 = t11176 * t3747;
    let t13546 = t9568 * t13315;
    let t13547 = t92 * t13546;
    let t13549 = t2404 * t13352;
    let t13550 = t92 * t13549;
    let t13552 = t2404 * t13320;
    let t13553 = t3051 * t13552;
    let t13555 = t2404 * t13309;
    let t13556 = t92 * t13555;
    let t13558 = t683 * t13346;
    let t13559 = t92 * t13558;
    let t13561 = t683 * t13301;
    let t13562 = t3051 * t13561;
    let t13564 = t683 * t13296;
    let t13565 = t92 * t13564;
    let t13567 = t665 * t668;
    (t13543, t13544, t13547, t13550, t13553, t13556, t13559, t13562, t13565, t13567)
}
