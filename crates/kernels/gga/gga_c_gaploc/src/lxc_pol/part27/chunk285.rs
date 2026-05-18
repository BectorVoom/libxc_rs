//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 285/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk285<F: Float>(t338: F, t13: F, t30: F, t1135: F) -> F {
    let t1154 = t338 * t338;
    let t1155 = F::new(1.0) / t1154;
    let t1156 = t13 * t1155;
    let t1157 = t30 * t30;
    let t1158 = F::new(1.0) / t1157;
    let t1159 = t1135 * t1158;
    let t1161 = F::new(0.16081824322151104822e2) * t1156 * t1159;
    t1161
}
