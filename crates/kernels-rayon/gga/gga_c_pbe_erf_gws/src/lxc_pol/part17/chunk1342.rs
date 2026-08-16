//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1342/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1342(t14692: f64, t3979: f64, t51967: f64, t2410: f64, t4164: f64, t51952: f64, t51954: f64, t51957: f64, t51958: f64, t51960: f64, t51964: f64, t54588: f64, t54593: f64, t54596: f64, t54598: f64, t54599: f64, t54605: f64, t54607: f64, t54613: f64) -> f64 {
    let t54616 = t3979 * t14692;
    let t54617 = 7.0_f64 / 2304.0_f64 * t54616;
    let t54619 = 35.0_f64 / 216.0_f64 * t51967;
    let t54620 = -t54588 / 768.0_f64 - t54593 / 384.0_f64 - t54596 / 48.0_f64 + t54598 * t54599 * t4164 * t2410 / 4.0_f64 - 5.0_f64 / 384.0_f64 * t54605 - t54607 / 96.0_f64 + 7.0_f64 / 72.0_f64 * t51952 + 7.0_f64 / 1152.0_f64 * t51954 + t51957 - 7.0_f64 / 288.0_f64 * t51958 + t54613 / 48.0_f64 + 7.0_f64 / 288.0_f64 * t51960 + t54617 - 35.0_f64 / 1152.0_f64 * t51964 - t54619;
    t54620
}
