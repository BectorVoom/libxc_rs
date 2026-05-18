//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 356/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk356<F: Float>(t1138: F, t1150: F, t1153: F, t1159: F, t1162: F, t1163: F, t1170: F, t1173: F, t1177: F, t1179: F) -> F {
    let t1182 = F::new(0.11360101276506094136e1) * t1150 * t1153 + t1159 + F::new(0.28977204965962526182e-1) * t1162 * t1163 + F::new(0.5848048239485271795e1) * t1170 * t1173 + t1177 + F::new(0.50380704458364197288e-2) * t1179 * t1138;
    t1182
}
