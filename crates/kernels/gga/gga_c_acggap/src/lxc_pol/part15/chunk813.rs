//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 813/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk813<F: Float>(t30546: F, t7570: F, t1106: F, t1992: F, t30147: F, t7586: F, t7478: F, t7799: F, t1004: F, t1966: F, t2100: F, t2104: F, t7630: F, t1035: F, t1979: F, t355: F, t864: F) -> (F, F, F, F, F, F, F, F) {
    let t30547 = t30546 * t7570;
    let t30559 = t30147 * t7586 * t1992 * t1106;
    let t30561 = t7799 * t7478;
    let t30567 = t1004 * t1966;
    let t30568 = t30567 * t2100;
    let t30570 = t7630 * t2104;
    let t30572 = t1035 * t1979;
    let t30573 = t355 * t864;
    (t30547, t30559, t30561, t30567, t30568, t30570, t30572, t30573)
}
