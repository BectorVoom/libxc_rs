//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1355/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1355(t4083: f64, t8743: f64, t54616: f64, t54621: f64, t15084: f64, t840: f64, t14311: f64, t14327: f64, t14911: f64, t2384: f64, t2388: f64, t2392: f64, t2498: f64, t51960: f64, t51964: f64, t51967: f64, t54624: f64, t54627: f64, t8616: f64) -> f64 {
    let t55884 = 7.0_f64 / 144.0_f64 * t8743 * t4083;
    let t55889 = 7.0_f64 / 1152.0_f64 * t54616;
    let t55892 = 35.0_f64 / 216.0_f64 * t54621;
    let t55901 = 7.0_f64 / 144.0_f64 * t840 * t15084;
    let t55903 = 7.0_f64 / 144.0_f64 * t51960 - t2384 * t14911 / 96.0_f64 + t55884 - t2388 * t14911 / 96.0_f64 - t2392 * t14911 / 96.0_f64 + t55889 - 35.0_f64 / 576.0_f64 * t51964 - 35.0_f64 / 108.0_f64 * t51967 - t55892 - t8616 * t4083 / 96.0_f64 - t2498 * t14311 / 48.0_f64 - t2498 * t14327 / 48.0_f64 - t54624 / 24.0_f64 + t55901 - t54627 / 24.0_f64;
    t55903
}
