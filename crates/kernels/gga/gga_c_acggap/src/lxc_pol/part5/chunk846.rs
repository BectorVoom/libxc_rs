//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 846/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk846<F: Float>(t1170: F, t13850: F, t3088: F, t3371: F, t3453: F, t1095: F, t1131: F, t384: F, t398: F, t879: F, t1164: F, t3266: F, t1037: F, t1165: F, t3451: F, t955: F) -> (F, F, F, F, F, F) {
    let t13851 = t1170 * t13850;
    let t13860 = t3088 * t3371;
    let t13861 = t13860 * t3453;
    let t13881 = t384 * t398 * t1095 * t879 * t1131;
    let t13889 = t1164 * t3266;
    let t13911 = t3451 * t1165 * t1037 * t955;
    (t13851, t13860, t13861, t13881, t13889, t13911)
}
