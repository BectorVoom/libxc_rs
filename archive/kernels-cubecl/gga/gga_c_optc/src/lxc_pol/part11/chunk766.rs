//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 766/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk766<F: Float>(t1499: F, t3079: F, t1502: F, t530: F, t1111: F, t1446: F, t2992: F, t1476: F, t3058: F, t1464: F, t2973: F, t2916: F) -> (F, F, F, F, F, F, F) {
    let t12119 = t1499 * t3079;
    let t12121 = t530 * t1502;
    let t12122 = t1111 * t12121;
    let t12168 = t1446 * t2992;
    let t12223 = t1476 * t3058;
    let t12238 = t1464 * t2973;
    let t12265 = t1476 * t2916;
    (t12119, t12121, t12122, t12168, t12223, t12238, t12265)
}
