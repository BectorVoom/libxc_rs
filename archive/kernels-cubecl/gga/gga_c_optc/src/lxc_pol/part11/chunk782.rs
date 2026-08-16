//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 782/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk782<F: Float>(t4744: F, t732: F, t193: F, t197: F, t4599: F, t745: F, t1256: F, t195: F, t1924: F, t4752: F, t1320: F, t3546: F) -> (F, F, F, F, F) {
    let t13526 = t732 * t4744;
    let t13536 = t193 * t745 * t4599 * t197;
    let t13538 = t195 * t1256;
    let t13543 = t193 * t1924 * t4752;
    let t13573 = t3546 * t1320;
    (t13526, t13536, t13538, t13543, t13573)
}
