//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 823/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk823<F: Float>(t435: F, t864: F, t1096: F, t1165: F, t12991: F, t3809: F, t388: F, t1084: F, t1181: F, t12936: F, t3646: F, t396: F, t409: F, t3206: F, t334: F, t339: F) -> (F, F, F, F, F, F, F, F) {
    let t12992 = t435 * t864;
    let t12995 = t12991 * t1165 * t12992 * t1096;
    let t12999 = t12991 * t1165 * t388 * t3809;
    let t13031 = t12936 * t1181 * t12992 * t1084;
    let t13039 = t3646 * t396;
    let t13040 = t13039 * t409;
    let t13064 = t3206 * t334;
    let t13065 = t13064 * t339;
    (t12992, t12995, t12999, t13031, t13039, t13040, t13064, t13065)
}
