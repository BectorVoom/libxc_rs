//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 417/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk417<F: Float>(t2610: F, t486: F, t1423: F, t723: F, t835: F, t2102: F, t773: F, t2086: F, t805: F, t119: F, t3831: F, t481: F) -> (F, F, F, F, F, F) {
    let t6118 = t486 * t2610;
    let t6119 = t1423 * t6118;
    let t6125 = t835 * t723;
    let t6141 = t773 * t2102;
    let t6148 = t805 * t2086;
    let t6305 = t481 * t3831 * t119;
    (t6118, t6119, t6125, t6141, t6148, t6305)
}
