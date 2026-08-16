//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 822/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk822<F: Float>(t1095: F, t1426: F, t9536: F, t598: F, t137: F, t1772: F, t1083: F, t1089: F, t1841: F, t2118: F, t2297: F, t535: F) -> (F, F, F, F, F, F, F) {
    let t9538 = t1426 * t1095 * t9536;
    let t9539 = t598 * t9538;
    let t9541 = t137 * t1772;
    let t9543 = t1089 * t1083 * t9541;
    let t9544 = t598 * t9543;
    let t9546 = t2118 * t1841;
    let t9549 = t1426 * t535 * t2297;
    (t9538, t9539, t9541, t9543, t9544, t9546, t9549)
}
