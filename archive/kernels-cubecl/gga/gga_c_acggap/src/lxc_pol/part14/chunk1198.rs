//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1198/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1198<F: Float>(t1881: F, t7614: F, t17912: F, t2302: F, t31443: F, t8906: F, t13287: F, t8402: F, t2001: F, t5956: F, t5961: F, t6205: F) -> (F, F, F, F, F, F) {
    let t40507 = t7614 * t1881;
    let t40511 = t31443 * t17912 * t2302 * t8906;
    let t40515 = t31443 * t13287 * t2302 * t8402;
    let t40517 = t2001 * t5956;
    let t40519 = t2001 * t5961;
    let t40521 = t2001 * t6205;
    (t40507, t40511, t40515, t40517, t40519, t40521)
}
