//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 902/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk902<F: Float>(t16988: F, t2672: F, t935: F, t313: F, t16644: F, t2722: F, t16225: F, t7865: F, t894: F, t16636: F, t3608: F, t7857: F) -> (F, F, F, F, F, F, F) {
    let t16990 = t16988 * t2672 * t935;
    let t16991 = t313 * t16990;
    let t16994 = t2722 * t16644;
    let t16997 = t7865 * t16225;
    let t16998 = t894 * t16997;
    let t17001 = t3608 * t16636;
    let t17004 = t7857 * t16225;
    (t16990, t16991, t16994, t16997, t16998, t17001, t17004)
}
