//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 877/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk877<F: Float>(t1992: F, t7585: F, t7842: F, t955: F, t7423: F, t7839: F, t1983: F, t7586: F, t3073: F, t7646: F, t3459: F, t1090: F, t30154: F) -> (F, F, F, F, F, F) {
    let t30353 = t7585 * t7842 * t1992 * t955;
    let t30355 = t7839 * t7423;
    let t30362 = t7585 * t7586 * t1983 * t955;
    let t30364 = t3073 * t7646;
    let t30365 = t30364 * t3459;
    let t30369 = t30154 * t7586 * t1992 * t1090;
    (t30353, t30355, t30362, t30364, t30365, t30369)
}
