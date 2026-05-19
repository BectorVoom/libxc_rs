//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 332/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk332<F: Float>(t1037: F, t1057: F, t1023: F, t1030: F) -> (F, F) {
    let t1059 = F::new(1.0) * t1037 * t1057;
    let t1060 = F::cast_from(0.17123333333333333333e-1_f64) * t1023;
    let t1062 = -t1060 - F::cast_from(0.17123333333333333333e-1_f64) * t1030;
    (t1059, t1062)
}
