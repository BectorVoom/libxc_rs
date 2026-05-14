//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 761/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk761<F: Float>(t2551: F, t735: F, t1069: F, t1617: F, t2729: F, t586: F, t213: F, t331: F, t34: F, t649: F, t641: F, t837: F, t2592: F, t639: F, t2597: F, t5493: F) -> (F, F, F, F, F, F, F) {
    let t6971 = 4.0 / 45.0 * t2551 * t735;
    let t6998 = t1069 * t1617;
    let t7011 = t2729 * t586;
    let t7018 = t331 * t213;
    let t7019 = t649 * t34;
    let t7039 = t837 * t641;
    let t7040 = t7039 * t2592;
    let t7041 = t639 * t7040;
    let t7043 = t5493 * t2597;
    (t6971, t6998, t7011, t7018, t7019, t7041, t7043)
}
