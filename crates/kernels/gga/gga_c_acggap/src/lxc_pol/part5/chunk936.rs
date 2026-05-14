//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 936/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk936<F: Float>(t12930: F, t1470: F, t1423: F, t3700: F, t3379: F, t4410: F, t12752: F, t1545: F, t1008: F, t4724: F, t14173: F, t4916: F, t3391: F, t4680: F, t4915: F, t1111: F, t1181: F, t15995: F) -> (F, F, F, F, F, F, F, F) {
    let t17694 = t12930 * t1470;
    let t17701 = t3700 * t1423;
    let t17703 = t3379 * t4410;
    let t17708 = t12752 * t1545;
    let t17710 = t1008 * t4724;
    let t17718 = t14173 * t4916;
    let t17721 = t3391 * t4680 * t4915;
    let t17725 = t3391 * t1181 * t15995 * t1111;
    (t17694, t17701, t17703, t17708, t17710, t17718, t17721, t17725)
}
