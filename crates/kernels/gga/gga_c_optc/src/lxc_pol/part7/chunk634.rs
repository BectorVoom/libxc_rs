//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 634/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk634<F: Float>(t310: F, t3137: F, t448: F, t1137: F, t2586: F, t1133: F, t1027: F, t381: F) -> (F, F, F, F, F) {
    let t3138 = t310 * t3137;
    let t3140 = F::cast_from(0.60369177012421929547e-3_f64) * t448 * t3138;
    let t3141 = t2586 * t1137;
    let t3142 = t1133 * t3141;
    let t3145 = F::new(1.0) / t381 / t1027;
    (t3138, t3140, t3141, t3142, t3145)
}
