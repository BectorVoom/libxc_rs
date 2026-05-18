//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 813/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk813<F: Float>(t40: F, t6930: F, t4: F, t959: F, t1448: F, t2551: F, t735: F, t1069: F, t1617: F, t2729: F, t586: F, t213: F, t331: F) -> (F, F, F, F, F, F) {
    let t6931 = t40 * t6930;
    let t6932 = F::new(2.0) * t6931;
    let t6967 = t959 * t4;
    let t6968 = t6967 * t1448;
    let t6971 = F::new(4.0) / F::new(45.0) * t2551 * t735;
    let t6998 = t1069 * t1617;
    let t7011 = t2729 * t586;
    let t7018 = t331 * t213;
    (t6932, t6968, t6971, t6998, t7011, t7018)
}
