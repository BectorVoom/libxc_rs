//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 398/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk398<F: Float>(t103: F, t133: F, t193: F, t197: F, t102: F, t745: F, t751: F, t616: F) -> (F, F, F, F) {
    let t1923 = F::new(1100.0) / F::new(81.0) * t193 * t133 * t103 * t197;
    let t1924 = t745 * t102;
    let t1926 = t193 * t1924 * t751;
    let t1928 = t616 * t616;
    (t1923, t1924, t1926, t1928)
}
