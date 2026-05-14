//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1034/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1034<F: Float>(t2001: F, t5961: F, t6205: F, t6211: F, t2118: F, t6215: F, t1998: F, t6194: F, t5878: F, t1988: F, t9538: F, t1095: F, t1426: F, t38922: F, t598: F, t13287: F, t2302: F, t31195: F, t8901: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40519 = t2001 * t5961;
    let t40521 = t2001 * t6205;
    let t40523 = t2001 * t6211;
    let t40525 = t2118 * t6215;
    let t40527 = t1998 * t6194;
    let t40529 = t2001 * t5878;
    let t40533 = t1988 * t9538;
    let t40537 = t598 * t1426 * t1095 * t38922;
    let t40542 = t31195 * t13287 * t2302 * t8901;
    (t40519, t40521, t40523, t40525, t40527, t40529, t40533, t40537, t40542)
}
