//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 916/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk916<F: Float>(t30364: F, t5147: F, t1992: F, t30692: F, t7842: F, t8901: F, t30689: F, t4967: F, t525: F, t864: F, t1165: F, t31567: F, t604: F, t23688: F, t7346: F, t7310: F, t8771: F) -> (F, F, F, F, F, F, F) {
    let t36006 = t30364 * t5147;
    let t36010 = t30692 * t7842 * t1992 * t8901;
    let t36017 = t30689 * t4967;
    let t36019 = t525 * t864;
    let t36022 = t31567 * t1165 * t604 * t36019;
    let t36030 = t7346 * t1165 * t604 * t23688;
    let t36032 = t7310 * t8771;
    (t36006, t36010, t36017, t36019, t36022, t36030, t36032)
}
