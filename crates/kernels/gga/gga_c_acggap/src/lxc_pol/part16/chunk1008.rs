//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1008/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1008<F: Float>(t1988: F, t9565: F, t1089: F, t3201: F, t598: F, t9563: F, t1083: F, t39219: F, t1980: F, t38893: F, t7458: F, t1861: F, t7614: F, t1998: F, t5826: F, t1165: F, t5651: F, t604: F, t8463: F) -> (F, F, F, F, F, F, F) {
    let t39771 = t1988 * t9565;
    let t39775 = t598 * t1089 * t3201 * t9563;
    let t39779 = t598 * t1089 * t1083 * t39219;
    let t39782 = t1980 * t7458 * t38893;
    let t39784 = t7614 * t1861;
    let t39786 = t1998 * t5826;
    let t39790 = t8463 * t1165 * t604 * t5651;
    (t39771, t39775, t39779, t39782, t39784, t39786, t39790)
}
