//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 626/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk626<F: Float>(t3450: F, t5942: F, t12968: F, t23455: F, t3455: F, t13140: F, t6695: F, t9099: F, t379: F, t6639: F, t9144: F, t574: F, t5935: F, t144: F, t26577: F, t3478: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26924 = t5942 * t3450;
    let t26925 = t12968 * t26924;
    let t26928 = t23455 * t3455;
    let t26929 = t13140 * t26928;
    let t26932 = t9099 * t6695;
    let t26935 = t6639 * t379;
    let t26936 = t9144 * t26935;
    let t26940 = t574 * t5935 * t3455;
    let t26943 = t144 * t26577;
    let t26947 = t574 * t5935 * t3478;
    (t26924, t26925, t26928, t26929, t26932, t26935, t26936, t26940, t26943, t26947)
}
