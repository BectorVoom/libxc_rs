//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 900/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk900<F: Float>(t2264: F, t30797: F, t7839: F, t8518: F, t8522: F, t31699: F, t8526: F, t4713: F, t7822: F, t7637: F, t8506: F, t137: F, t4099: F, t1426: F, t368: F, t598: F) -> (F, F, F, F, F, F, F, F) {
    let t34033 = t30797 * t2264;
    let t34035 = t7839 * t8518;
    let t34037 = t7839 * t8522;
    let t34039 = t31699 * t8526;
    let t34041 = t7822 * t4713;
    let t34043 = t7637 * t8506;
    let t34045 = t137 * t4099;
    let t34048 = t598 * t1426 * t368 * t34045;
    (t34033, t34035, t34037, t34039, t34041, t34043, t34045, t34048)
}
