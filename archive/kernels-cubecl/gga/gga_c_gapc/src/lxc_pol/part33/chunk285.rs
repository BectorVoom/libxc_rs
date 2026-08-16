//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 285/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk285<F: Float>(t1062: F, t1063: F, t1052: F, t1059: F, t125: F, t902: F, t311: F) -> (F, F, F) {
    let t1064 = t1062 * t1063;
    let t1066 = F::cast_from(0.28183154870449698953e-3_f64) * t1052 + F::cast_from(0.41036913933938047292e-5_f64) * t1059 - F::cast_from(0.58714905980103539485e-5_f64) * t1064;
    let t1068 = t902 * t125;
    let t1069 = t311 * t1068;
    (t1066, t1068, t1069)
}
