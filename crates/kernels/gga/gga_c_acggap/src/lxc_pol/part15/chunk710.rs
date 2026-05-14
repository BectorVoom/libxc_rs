//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 710/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk710<F: Float>(t2268: F, t7433: F, t2264: F, t7839: F, t1511: F, t570: F, t1526: F, t1298: F, t579: F, t336: F, t2046: F, t1181: F, t1454: F, t7351: F, t7564: F, t137: F, t3706: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8580 = t7433 * t2268;
    let t8582 = t7839 * t2264;
    let t8584 = t570 * t1511;
    let t8586 = t570 * t1526;
    let t8588 = t579 * t1298;
    let t8589 = t336 * t8588;
    let t8590 = t2046 * t8589;
    let t8597 = t1181 * t7351 * t1454;
    let t8598 = t7564 * t8597;
    let t8600 = t3706 * t137;
    (t8580, t8582, t8584, t8586, t8589, t8590, t8597, t8598, t8600)
}
