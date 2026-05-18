//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 326/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk326<F: Float>(t1038: F, t1040: F, t1023: F, t373: F, t222: F, t381: F, t790: F) -> (F, F, F, F, F) {
    let t1041 = t1038 * t1040;
    let t1043 = F::new(0.29896666666666666667e0) * t1023;
    let t1045 = f64::sqrt(t373);
    let t1046 = t1045 * t1040;
    let t1049 = t222 * t790 * t381;
    (t1041, t1043, t1045, t1046, t1049)
}
