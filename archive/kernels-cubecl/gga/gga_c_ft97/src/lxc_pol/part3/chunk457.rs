//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 457/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk457<F: Float>(t1009: F, t1995: F, t1008: F, t549: F, t554: F, t2007: F, t929: F, t120: F, t383: F, t3056: F, t528: F, t72: F) -> (F, F, F, F, F, F, F, F) {
    let t3350 = t1995 * t1009;
    let t3355 = t549 * t1008;
    let t3356 = t3355 * t554;
    let t3359 = t2007 * t929;
    let t3360 = t120 * t383;
    let t3363 = t528 * t3056;
    let t3364 = t3363 * t120;
    let t3366 = t929 * t383;
    let t3368 = t72 * t3366 * t120;
    (t3350, t3355, t3356, t3359, t3360, t3363, t3364, t3368)
}
