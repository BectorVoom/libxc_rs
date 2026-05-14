//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 861/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk861<F: Float>(t1998: F, t3531: F, t7528: F, t7637: F, t2109: F, t7610: F, t1980: F, t31025: F, t7458: F, t31362: F, t7339: F, t1101: F, t1983: F, t30827: F, t7586: F, t1181: F, t3346: F, t599: F, t7493: F) -> (F, F, F, F, F, F, F) {
    let t31680 = t1998 * t3531;
    let t31682 = t7637 * t7528;
    let t31684 = t7610 * t2109;
    let t31687 = t1980 * t7458 * t31025;
    let t31689 = t31362 * t7339;
    let t31693 = t30827 * t7586 * t1983 * t1101;
    let t31697 = t7493 * t1181 * t599 * t3346;
    (t31680, t31682, t31684, t31687, t31689, t31693, t31697)
}
