//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 856/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk856<F: Float>(t1114: F, t19776: F, t409: F, t7996: F, t4358: F, t960: F, t2840: F, t4835: F, t2506: F, t4813: F, t40: F, t4742: F, t959: F, t414: F, t4560: F, t4859: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22493 = t1114 * t19776;
    let t22590 = t409 * t7996;
    let t22592 = t4358 * t960;
    let t22594 = t2840 * t4835;
    let t22599 = t2506 * t4813;
    let t22606 = t40 * t959 * t4742;
    let t22609 = t414 * t7996;
    let t22634 = t4560 * t960;
    let t22636 = t4859 * t960;
    (t22493, t22590, t22592, t22594, t22599, t22606, t22609, t22634, t22636)
}
