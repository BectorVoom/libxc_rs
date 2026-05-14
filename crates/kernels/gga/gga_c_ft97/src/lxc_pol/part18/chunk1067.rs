//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1067/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1067<F: Float>(t3539: F, t582: F, t1045: F, t9114: F, t616: F, t9132: F, t157: F, t1984: F, t2097: F, t2179: F, t525: F, t9438: F, t1985: F, t597: F, t2142: F, t2101: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t49614 = t582 * t3539;
    let t49634 = t9114 * t1045;
    let t50229 = t9132 * t616;
    let t50235 = t1984 * t157;
    let t50240 = t2097 * t2179;
    let t50249 = t525 * t157;
    let t50260 = t1045 * t9438;
    let t50268 = t1985 * t597;
    let t50287 = t582 * t2142;
    let t50550 = t2101 * t616;
    (t49614, t49634, t50229, t50235, t50240, t50249, t50260, t50268, t50287, t50550)
}
