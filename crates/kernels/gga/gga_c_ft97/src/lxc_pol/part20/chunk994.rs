//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 994/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk994<F: Float>(t4125: F, t811: F, t2035: F, t2687: F, t39: F, t2719: F, t283: F, t19106: F, t4092: F, t28676: F, t70456: F, t15128: F, t848: F, t2681: F, t2843: F, t1240: F, t799: F) -> (F, F, F, F, F, F, F, F) {
    let t70598 = t811 * t4125;
    let t70677 = t2687 * t39 * t2035;
    let t70683 = t2719 * t283;
    let t70779 = t4092 * t19106;
    let t70786 = t28676 * t70456;
    let t72163 = t848 * t15128;
    let t72190 = t2681 * t2843;
    let t72397 = t799 * t1240;
    (t70598, t70677, t70683, t70779, t70786, t72163, t72190, t72397)
}
