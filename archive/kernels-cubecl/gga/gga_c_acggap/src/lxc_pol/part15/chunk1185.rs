//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1185/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1185<F: Float>(t13287: F, t2302: F, t31443: F, t8402: F, t2001: F, t5956: F, t5961: F, t6205: F, t6211: F, t2118: F, t6215: F, t1998: F, t6194: F) -> (F, F, F, F, F, F, F) {
    let t40515 = t31443 * t13287 * t2302 * t8402;
    let t40517 = t2001 * t5956;
    let t40519 = t2001 * t5961;
    let t40521 = t2001 * t6205;
    let t40523 = t2001 * t6211;
    let t40525 = t2118 * t6215;
    let t40527 = t1998 * t6194;
    (t40515, t40517, t40519, t40521, t40523, t40525, t40527)
}
