//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 824/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk824<F: Float>(t1170: F, t1171: F, t30538: F, t1108: F, t7746: F, t1113: F, t14046: F, t7336: F, t7643: F, t1973: F, t7630: F, t1985: F, t30231: F, t7637: F, t7796: F, t1004: F, t390: F, t7613: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30948 = t1170 * t30538 * t1171;
    let t30978 = t7746 * t1108;
    let t30982 = t7746 * t1113;
    let t30984 = t14046 * t7336;
    let t30985 = t30984 * t7643;
    let t30987 = t7630 * t1973;
    let t30989 = t30231 * t1985;
    let t30993 = t7637 * t7796;
    let t31001 = t1004 * t7613 * t390;
    (t30948, t30978, t30982, t30984, t30985, t30987, t30989, t30993, t31001)
}
