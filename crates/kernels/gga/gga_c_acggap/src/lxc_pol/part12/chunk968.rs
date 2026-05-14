//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 968/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk968<F: Float>(t7839: F, t8962: F, t1077: F, t1181: F, t2068: F, t525: F, t604: F, t8966: F, t33953: F, t5284: F, t13299: F, t31115: F, t31195: F, t35420: F, t31276: F, t8875: F) -> (F, F, F, F, F, F, F) {
    let t35623 = t7839 * t8962;
    let t35629 = t2068 * t1181 * t604 * t525 * t1077;
    let t35631 = t7839 * t8966;
    let t35633 = t33953 * t5284;
    let t35635 = t31115 * t13299 * t35633;
    let t35638 = t31195 * t13299 * t35420;
    let t35643 = t31276 * t8875;
    (t35623, t35629, t35631, t35633, t35635, t35638, t35643)
}
