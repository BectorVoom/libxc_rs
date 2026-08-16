//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2374/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2374<F: Float>(t10868: F, t2482: F, t27: F, t820: F, t823: F, t9948: F, t839: F, t2681: F, t2719: F, t10111: F, t9720: F, t685: F, t827: F, t837: F) -> (F, F, F, F, F, F) {
    let t40352 = t2482 * t10868 * t27;
    let t40360 = t820 * t823 * t9948;
    let t40361 = t40360 * t839;
    let t40398 = t820 * t2719 * t2681;
    let t40406 = t10111 * t823 * t9720;
    let t40409 = t40406 * t827 * t685 * t837;
    (t40352, t40360, t40361, t40398, t40406, t40409)
}
