//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1268/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1268(t4083: f64, t8743: f64, t54616: f64, t15084: f64, t840: f64, t2242: f64, t4230: f64, t15027: f64, t9270: f64, t15089: f64, t4414: f64, t14924: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t55884 = 7.0_f64 / 144.0_f64 * t8743 * t4083;
    let t55889 = 7.0_f64 / 1152.0_f64 * t54616;
    let t55901 = 7.0_f64 / 144.0_f64 * t840 * t15084;
    let t55904 = t2242 * t4230;
    let t55918 = 7.0_f64 / 72.0_f64 * t9270 * t15027;
    let t55936 = 7.0_f64 / 72.0_f64 * t4414 * t15089;
    let t55942 = 7.0_f64 / 72.0_f64 * t4414 * t14924;
    (t55884, t55889, t55901, t55904, t55918, t55936, t55942)
}
