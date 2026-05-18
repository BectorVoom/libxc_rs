//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1339/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1339<F: Float>(t1081: F, t8749: F, t3029: F, t3058: F, t8697: F, t26224: F, t406: F, t26214: F, t1066: F, t8882: F, t2927: F, t2973: F) -> (F, F, F, F, F, F, F) {
    let t26722 = t1081 * t8749;
    let t26732 = t3029 * t3058;
    let t26735 = t1081 * t8697;
    let t26738 = t406 * t26224;
    let t26745 = t406 * t26214;
    let t26749 = t8882 * t1066;
    let t26754 = t2927 * t2973;
    (t26722, t26732, t26735, t26738, t26745, t26749, t26754)
}
