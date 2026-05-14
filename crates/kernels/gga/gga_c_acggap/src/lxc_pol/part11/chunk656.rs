//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 656/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk656<F: Float>(t1980: F, t7528: F, t1988: F, t2113: F, t1131: F, t137: F, t1089: F, t1459: F, t598: F, t1004: F, t597: F) -> (F, F, F, F, F, F) {
    let t7529 = t1980 * t7528;
    let t7531 = t1988 * t2113;
    let t7533 = t137 * t1131;
    let t7535 = t1089 * t1459 * t7533;
    let t7536 = t598 * t7535;
    let t7538 = t1004 * t597;
    (t7529, t7531, t7533, t7535, t7536, t7538)
}
