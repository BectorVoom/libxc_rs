//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 922/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk922<F: Float>(t12727: F, t1558: F, t13298: F, t13364: F, t1421: F, t3169: F, t13299: F, t3176: F, t13285: F, t3073: F, t1101: F, t176: F, t406: F, t8790: F, t13292: F, t1106: F) -> (F, F, F, F, F, F, F) {
    let t17148 = t12727 * t1558;
    let t17152 = t13298 * t13364 * t1421 * t3169;
    let t17156 = t13298 * t13299 * t1421 * t3176;
    let t17167 = t3073 * t13285;
    let t17171 = t17167 * t176 * t8790 * t1101 * t406;
    let t17173 = t3073 * t13292;
    let t17177 = t17173 * t13364 * t8790 * t1106 * t406;
    (t17148, t17152, t17156, t17167, t17171, t17173, t17177)
}
