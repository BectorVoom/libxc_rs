//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 387/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk387<F: Float>(t103: F, t133: F, t193: F, t197: F, t102: F, t745: F, t48: F, t53: F, t539: F, t592: F, t544: F, t171: F) -> (F, F, F, F, F, F, F) {
    let t1923 = F::cast_from(1100.0_f64) / F::cast_from(81.0_f64) * t193 * t133 * t103 * t197;
    let t1924 = t745 * t102;
    let t1933 = F::cast_from(1.0_f64) / t48;
    let t1940 = F::cast_from(1.0_f64) / t53;
    let t1966 = F::cast_from(8.0_f64) * t539 * t592;
    let t1968 = F::cast_from(8.0_f64) * t544 * t592;
    let t1974 = t171 * t171;
    (t1923, t1924, t1933, t1940, t1966, t1968, t1974)
}
