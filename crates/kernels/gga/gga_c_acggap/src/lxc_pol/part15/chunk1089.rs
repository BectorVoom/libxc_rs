//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1089/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1089<F: Float>(t1083: F, t1772: F, t1980: F, t355: F, t7458: F, t1841: F, t7712: F, t1967: F, t9565: F, t1410: F, t525: F, t1181: F, t2068: F, t599: F) -> (F, F, F, F, F) {
    let t38815 = t1980 * t7458 * t1083 * t355 * t1772;
    let t38817 = t7712 * t1841;
    let t38820 = t1967 * t9565;
    let t38827 = t525 * t1410;
    let t38830 = t2068 * t1181 * t599 * t38827;
    (t38815, t38817, t38820, t38827, t38830)
}
