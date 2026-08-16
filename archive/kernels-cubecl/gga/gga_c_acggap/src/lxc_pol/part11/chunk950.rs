//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 950/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk950<F: Float>(t2087: F, t7630: F, t1160: F, t30539: F, t1167: F, t30268: F, t7339: F, t1165: F, t12816: F, t7351: F, t7493: F, t1998: F, t3493: F) -> (F, F, F, F, F, F) {
    let t31629 = t7630 * t2087;
    let t31631 = t1160 * t30539;
    let t31632 = t31631 * t1167;
    let t31634 = t30268 * t7339;
    let t31638 = t7493 * t1165 * t7351 * t12816;
    let t31640 = t1998 * t3493;
    (t31629, t31631, t31632, t31634, t31638, t31640)
}
