//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1141/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1141<F: Float>(t1988: F, t9565: F, t1089: F, t3201: F, t598: F, t9563: F, t1083: F, t39219: F, t1980: F, t38893: F, t7458: F, t1861: F, t7614: F) -> (F, F, F, F, F) {
    let t39771 = t1988 * t9565;
    let t39775 = t598 * t1089 * t3201 * t9563;
    let t39779 = t598 * t1089 * t1083 * t39219;
    let t39782 = t1980 * t7458 * t38893;
    let t39784 = t7614 * t1861;
    (t39771, t39775, t39779, t39782, t39784)
}
