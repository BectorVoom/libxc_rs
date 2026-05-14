//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1005/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1005<F: Float>(t19824: F, t184: F, t5418: F, t16388: F, t2583: F, t149: F, t5224: F, t63: F, t1041: F, t17095: F, t6891: F, t6895: F, t167: F, t168: F, t17033: F, t16421: F, t2591: F) -> (F, F, F, F, F, F, F, F) {
    let t19825 = 12.0 * t19824;
    let t19873 = t184 * t5418;
    let t19909 = t16388 * t2583;
    let t19910 = 35.0 / 24.0 * t19909;
    let t19932 = t149 * t5224 * t63;
    let t19947 = t17095 * t1041;
    let t20010 = t6895 * t6891;
    let t20060 = t167 * t168 * t17033;
    let t20065 = t16421 * t168 * t2591;
    (t19825, t19873, t19910, t19932, t19947, t20010, t20060, t20065)
}
