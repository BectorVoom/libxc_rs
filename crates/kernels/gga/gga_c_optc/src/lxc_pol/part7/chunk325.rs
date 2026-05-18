//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 325/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk325<F: Float>(t1032: F, t389: F, t385: F, t375: F, t376: F, t1023: F, t1030: F) -> (F, F, F, F, F, F) {
    let t1034 = F::new(0.62182e-1) * t1032 * t389;
    let t1035 = t385 * t385;
    let t1036 = F::new(1.0) / t1035;
    let t1037 = t375 * t1036;
    let t1038 = F::new(1.0) / t376;
    let t1040 = -t1023 / F::new(3.0) - t1030 / F::new(3.0);
    (t1034, t1035, t1036, t1037, t1038, t1040)
}
