//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 805/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk805<F: Float>(t3247: F, t32692: F, t9647: F, t10697: F, t9624: F, t2558: F, t33348: F, t123: F, t33137: F, t2563: F, t13182: F, t1841: F) -> (F, F, F, F, F) {
    let t42936 = t9647 * t32692 * t3247;
    let t42939 = t9647 * t10697 * t9624;
    let t42942 = t9647 * t33348 * t2558;
    let t42949 = t33137 * t123;
    let t42951 = t9647 * t42949 * t2563;
    let t42953 = t1841 * t13182;
    (t42936, t42939, t42942, t42951, t42953)
}
