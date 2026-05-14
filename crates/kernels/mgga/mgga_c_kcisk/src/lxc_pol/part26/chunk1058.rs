//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1058/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1058<F: Float>(t27116: F, t27119: F, t27121: F, t27124: F, t27127: F, t27131: F, t27134: F, t27136: F, t27138: F, t27141: F, t27143: F, t27147: F, t27149: F, t27153: F, t27155: F, t27158: F, t27162: F, t27164: F, t27166: F) -> (F,) {
    let t28094 = -0.9375e-1 * t27116 + 0.71944444444444444443e-1 * t27119 + 0.33333333333333333333e0 * t27121 - 0.5625e0 * t27124 + 0.25e0 * t27127 + 0.101171875e-1 * t27131 - 0.4046875e-1 * t27134 + 0.26979166666666666666e-1 * t27136 - 0.13489583333333333333e-1 * t27138 + 0.26979166666666666666e-1 * t27141 - 0.20833333333333333333e-1 * t27143 - 0.13489583333333333333e-1 * t27147 - 0.13489583333333333333e-1 * t27149 - 0.9375e-1 * t27153 + 0.625e-1 * t27155 + 0.55555555555555555555e-1 * t27158 + 0.625e-1 * t27162 - 0.125e0 * t27164 - 0.20234375e-1 * t27166;
    (t28094,)
}
