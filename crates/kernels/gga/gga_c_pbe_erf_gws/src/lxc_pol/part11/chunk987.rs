//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 987/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk987<F: Float>(t1019: F, t12452: F, t18237: F, t34500: F, t43029: F, t47832: F, t47836: F, t47839: F, t47841: F, t47844: F, t47848: F, t47850: F, t47851: F, t1010: F, t40493: F, t12440: F, t30630: F) -> (F, F, F, F) {
    let t47855 = 16.0 / 5.0 * t12452 * t1019;
    let t47856 = -t47832 - t47836 + t47839 - t47841 - t47844 - t47848 - t47850 - t47851 + 8.0 / 9.0 * t43029 + 8.0 / 3.0 * t34500 - t47855 + t18237;
    let t47862 = 16.0 / 45.0 * t40493 * t1010;
    let t47864 = 16.0 / 5.0 * t30630 * t12440;
    (t47855, t47856, t47862, t47864)
}
