//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 940/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk940<F: Float>(t17550: F, t4469: F, t13092: F, t4269: F, t3431: F, t4701: F, t13850: F, t3360: F, t1165: F, t3491: F, t530: F, t1507: F, t3573: F, t13064: F, t500: F, t171: F, t3300: F) -> (F, F, F, F, F, F, F, F) {
    let t17870 = t17550 * t4469;
    let t17876 = t13092 * t4269;
    let t17886 = t3431 * t4701;
    let t17888 = t3360 * t13850;
    let t17891 = t17888 * t1165 * t530 * t3491;
    let t17895 = t3573 * t1507;
    let t17902 = t13064 * t500;
    let t17912 = t171 * t3300;
    (t17870, t17876, t17886, t17888, t17891, t17895, t17902, t17912)
}
