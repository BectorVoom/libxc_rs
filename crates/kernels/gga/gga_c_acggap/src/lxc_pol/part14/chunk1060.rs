//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1060/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1060<F: Float>(t1988: F, t9538: F, t1095: F, t1426: F, t38922: F, t598: F, t13287: F, t2302: F, t31195: F, t8901: F, t1782: F, t1992: F, t2095: F, t7426: F, t8480: F, t8605: F) -> (F, F, F, F, F) {
    let t40533 = t1988 * t9538;
    let t40537 = t598 * t1426 * t1095 * t38922;
    let t40542 = t31195 * t13287 * t2302 * t8901;
    let t40546 = t2095 * t1992 * t1782;
    let t40549 = t7426 * t8480 * t8605;
    (t40533, t40537, t40542, t40546, t40549)
}
