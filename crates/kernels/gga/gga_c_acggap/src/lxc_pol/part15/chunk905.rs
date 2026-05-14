//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 905/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk905<F: Float>(t31421: F, t1992: F, t7585: F, t7842: F, t8402: F, t7433: F, t8787: F, t31362: F, t8956: F, t7839: F, t8962: F, t8966: F, t33953: F, t5284: F, t13299: F, t31115: F) -> (F, F, F, F, F, F, F, F) {
    let t35603 = 0.22921875e-1 * t31421;
    let t35608 = t7585 * t7842 * t1992 * t8402;
    let t35610 = t7433 * t8787;
    let t35616 = t31362 * t8956;
    let t35623 = t7839 * t8962;
    let t35631 = t7839 * t8966;
    let t35633 = t33953 * t5284;
    let t35635 = t31115 * t13299 * t35633;
    (t35603, t35608, t35610, t35616, t35623, t35631, t35633, t35635)
}
