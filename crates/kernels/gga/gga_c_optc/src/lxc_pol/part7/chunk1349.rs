//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1349/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1349<F: Float>(t3117: F, t8914: F, t438: F, t935: F, t1028: F, t19: F, t3105: F, t3145: F, t2849: F, t3107: F, t123: F, t1897: F) -> (F, F, F, F, F, F) {
    let t26880 = t3117 * t8914;
    let t26881 = t935 * t438;
    let t26882 = t26881 * t1028;
    let t26887 = t3145 * t3105 * t19;
    let t26888 = t3107 * t2849;
    let t26889 = t1897 * t123;
    (t26880, t26881, t26882, t26887, t26888, t26889)
}
