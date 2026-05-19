//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 915/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk915<F: Float>(t14046: F, t7336: F, t7643: F, t1973: F, t7630: F, t1985: F, t30231: F, t1967: F, t7792: F, t7637: F, t7796: F, t1980: F, t1982: F, t1992: F, t5: F, t965: F) -> (F, F, F, F, F, F, F) {
    let t30984 = t14046 * t7336;
    let t30985 = t30984 * t7643;
    let t30987 = t7630 * t1973;
    let t30989 = t30231 * t1985;
    let t30990 = F::cast_from(0.28582678745379824648e-2_f64) * t30989;
    let t30991 = t1967 * t7792;
    let t30993 = t7637 * t7796;
    let t30998 = t1980 * t1982 * t5 * t965 * t1992;
    (t30984, t30985, t30987, t30990, t30991, t30993, t30998)
}
