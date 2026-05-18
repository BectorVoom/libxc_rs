//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 389/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk389<F: Float>(t178: F, t1882: F, t172: F, t5: F, t4: F, t144: F, t152: F, t512: F, t611: F, t641: F, t1044: F, t6: F) -> (F, F, F, F, F) {
    let t1883 = t178 * t1882;
    let t1886 = t172 * t5;
    let t1887 = t1886 * t4;
    let t1888 = t512 * t144 * t152 * t1887;
    let t1891 = t611 * t641;
    let t1892 = t1044 * t144;
    let t1894 = t172 * t6;
    (t1883, t1888, t1891, t1892, t1894)
}
