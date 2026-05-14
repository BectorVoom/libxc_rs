//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 810/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk810<F: Float>(t1240: F, t2681: F, t1200: F, t7606: F, t19106: F, t800: F, t4092: F, t2843: F, t799: F, t66422: F, t688: F, t17818: F, t17817: F, t65693: F, t1613: F, t1689: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t69996 = t2681 * t1240;
    let t70497 = t1200 * t7606;
    let t70550 = t800 * t19106;
    let t70779 = t4092 * t19106;
    let t72190 = t2681 * t2843;
    let t72397 = t799 * t1240;
    let t79528 = t66422 * t688;
    let t79529 = t79528 * t17818;
    let t79641 = t17817 * t65693;
    let t79931 = t1689 * t1613;
    (t69996, t70497, t70550, t70779, t72190, t72397, t79528, t79529, t79641, t79931)
}
