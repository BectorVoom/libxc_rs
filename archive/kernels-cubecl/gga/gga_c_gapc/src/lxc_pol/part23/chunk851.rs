//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 851/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk851<F: Float>(t122: F, t2153: F, t1034: F, t1089: F, t3364: F, t3368: F, t103: F, t2188: F, t1088: F, t1085: F, t3072: F, t3363: F) -> (F, F, F, F, F, F) {
    let t9906 = t2153 * t122;
    let t9907 = t9906 * t1034;
    let t9908 = t9907 * t1089;
    let t9910 = t3364 * t3368;
    let t9912 = t103 * t2188;
    let t9913 = t9912 * t1088;
    let t9914 = t1085 * t9913;
    let t9916 = t3363 * t3072;
    (t9906, t9908, t9910, t9913, t9914, t9916)
}
