//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 579/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk579<F: Float>(t186: F, t2723: F, t211: F, t1033: F, t663: F, t209: F, t617: F, t184: F, t1024: F, t1730: F, t1: F, t331: F) -> (F, F, F, F, F, F, F, F) {
    let t2724 = t186 * t2723;
    let t2726 = 2.0 / 15.0 * t211 * t2724;
    let t2728 = 2.0 / 15.0 * t1033 * t663;
    let t2729 = t617 * t209;
    let t2730 = t2729 * t184;
    let t2732 = 4.0 / 15.0 * t2730 * t1024;
    let t2734 = 4.0 / 15.0 * t1730 * t1024;
    let t2735 = t1 * t331;
    (t2724, t2726, t2728, t2729, t2730, t2732, t2734, t2735)
}
