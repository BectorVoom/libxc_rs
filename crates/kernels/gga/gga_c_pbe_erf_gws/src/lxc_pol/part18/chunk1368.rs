//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1368/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1368<F: Float>(t14138: F, t57508: F, t2306: F, t331: F, t3780: F, t3074: F, t833: F, t15177: F, t3979: F, t15275: F, t840: F, t15213: F, t4414: F) -> (F, F, F, F, F) {
    let t57509 = t57508 * t14138;
    let t57512 = t2306 * t3780 * t331;
    let t57514 = t3074 * t57512 * t833;
    let t57516 = t3979 * t15177;
    let t57518 = t840 * t15275;
    let t57534 = t4414 * t15213;
    (t57509, t57514, t57516, t57518, t57534)
}
