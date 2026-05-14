//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 531/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk531<F: Float>(t2606: F, t2670: F, t2752: F, t2829: F, t1076: F, t153: F, t542: F, t1220: F, t1278: F, t1288: F, t1296: F, t1328: F, t1335: F, t1338: F, t1426: F, t1431: F, t1450: F, t2449: F, t2476: F, t2508: F) -> (F, F, F) {
    let t2831 = t2606 + t2670 + t2752 + t2829;
    let t2837 = t153 * t542 * t1076;
    let t2839 = t1220 + t1328 + t1335 - t1338 + t1426 - t2449 + t1450 - t1278 + t1288 + t1296 + t2476 - t2508 - t1431;
    (t2831, t2837, t2839)
}
