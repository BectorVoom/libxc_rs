//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 860/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk860<F: Float>(t1101: F, t1983: F, t30827: F, t7586: F, t1181: F, t3346: F, t599: F, t7493: F, t3378: F, t7584: F, t7588: F, t2074: F, t30797: F, t7844: F, t1111: F, t1992: F, t30147: F) -> (F, F, F, F, F, F, F) {
    let t31693 = t30827 * t7586 * t1983 * t1101;
    let t31697 = t7493 * t1181 * t599 * t3346;
    let t31699 = t3378 * t7584;
    let t31700 = t31699 * t7588;
    let t31702 = t30797 * t2074;
    let t31704 = t31699 * t7844;
    let t31708 = t30147 * t7586 * t1992 * t1111;
    (t31693, t31697, t31699, t31700, t31702, t31704, t31708)
}
