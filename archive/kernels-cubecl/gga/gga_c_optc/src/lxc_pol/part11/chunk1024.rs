//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1024/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1024<F: Float>(t2086: F, t56: F, t111: F, t166: F, t6975: F, t145: F, t146: F, t622: F, t7000: F, t155: F, t6165: F, t693: F) -> (F, F, F, F, F) {
    let t22895 = t56 * t2086;
    let t22896 = t111 * t22895;
    let t22932 = F::cast_from(1.0_f64) / t6975 / t166;
    let t22933 = t145 * t22932;
    let t23013 = t146 * t7000 * t622;
    let t23017 = t155 * t693 * t6165;
    (t22895, t22896, t22933, t23013, t23017)
}
