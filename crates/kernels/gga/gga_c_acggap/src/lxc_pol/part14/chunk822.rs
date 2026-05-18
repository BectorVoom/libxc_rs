//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 822/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk822<F: Float>(t1083: F, t1089: F, t9563: F, t598: F, t1861: F, t2001: F, t1851: F, t3300: F, t9552: F, t1095: F, t4352: F, t9529: F) -> (F, F, F, F, F, F, F) {
    let t9565 = t1089 * t1083 * t9563;
    let t9566 = t598 * t9565;
    let t9568 = t2001 * t1861;
    let t9570 = t2001 * t1851;
    let t9573 = t1089 * t3300 * t9552;
    let t9574 = t598 * t9573;
    let t9577 = t4352 * t1095 * t9529;
    (t9565, t9566, t9568, t9570, t9573, t9574, t9577)
}
