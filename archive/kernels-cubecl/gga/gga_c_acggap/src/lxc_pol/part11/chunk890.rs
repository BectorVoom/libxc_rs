//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 890/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk890<F: Float>(t30546: F, t7570: F, t1106: F, t1992: F, t30147: F, t7586: F, t7478: F, t7799: F, t3176: F, t7585: F, t1004: F, t1966: F) -> (F, F, F, F, F) {
    let t30547 = t30546 * t7570;
    let t30559 = t30147 * t7586 * t1992 * t1106;
    let t30561 = t7799 * t7478;
    let t30565 = t7585 * t7586 * t1992 * t3176;
    let t30567 = t1004 * t1966;
    (t30547, t30559, t30561, t30565, t30567)
}
