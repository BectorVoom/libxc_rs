//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 861/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk861<F: Float>(t1264: F, t316: F, t449: F, t879: F, t3034: F, t309: F, t180: F, t3923: F, t3035: F, t441: F, t3912: F, t852: F) -> (F, F, F, F, F) {
    let t12271 = t316 * t449 * t879 * t1264;
    let t12273 = t309 * t3034;
    let t12276 = F::new(0.15805078039045227836e2) * t12273 * t180 * t3923;
    let t12278 = t3035 * t441 * t3923;
    let t12281 = F::new(0.26341796731742046395e1) * t852 * t3912;
    (t12271, t12273, t12276, t12278, t12281)
}
