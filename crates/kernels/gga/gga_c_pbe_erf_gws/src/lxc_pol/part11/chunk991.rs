//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 991/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk991<F: Float>(t1033: F, t12871: F, t1896: F, t47409: F, t587: F, t590: F, t1661: F, t1664: F, t10843: F, t3531: F, t32019: F, t3513: F, t12440: F, t7527: F, t12444: F, t1044: F, t1620: F, t1621: F, t40687: F) -> (F, F, F, F, F, F, F, F) {
    let t47906 = 8.0 / 15.0 * t1033 * t12871;
    let t47910 = 8.0 / 15.0 * t587 * t590 * t1896 * t47409;
    let t47914 = 4.0 / 9.0 * t587 * t1661 * t1664 * t47409;
    let t47916 = 16.0 / 9.0 * t10843 * t3531;
    let t47918 = 16.0 / 5.0 * t32019 * t3513;
    let t47920 = 16.0 / 5.0 * t7527 * t12440;
    let t47922 = 16.0 / 5.0 * t7527 * t12444;
    let t47926 = 16.0 / 15.0 * t1620 * t1621 * t40687 * t1044;
    (t47906, t47910, t47914, t47916, t47918, t47920, t47922, t47926)
}
