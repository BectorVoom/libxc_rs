//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 906/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk906<F: Float>(t30817: F, t7836: F, t1190: F, t30540: F, t30219: F, t7867: F, t7871: F, t1165: F, t3346: F, t604: F, t7493: F, t2070: F, t30792: F) -> (F, F, F, F, F, F) {
    let t30840 = t30817 * t7836;
    let t30844 = t30540 * t1190;
    let t30846 = t30219 * t7867;
    let t30848 = t30219 * t7871;
    let t30852 = t7493 * t1165 * t604 * t3346;
    let t30854 = t30792 * t2070;
    (t30840, t30844, t30846, t30848, t30852, t30854)
}
