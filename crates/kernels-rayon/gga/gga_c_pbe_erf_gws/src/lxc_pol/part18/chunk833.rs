//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 833/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk833(t4358: f64, t2626: f64, t5018: f64, t1820: f64, t1648: f64, t2643: f64, t2602: f64, t5493: f64, t639: f64, t2631: f64, t587: f64, t589: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7907 = 12.0_f64 * t4358;
    let t7913 = t5018 * t2626;
    let t7915 = 16.0_f64 / 45.0_f64 * t1820 * t7913;
    let t7919 = 16.0_f64 / 135.0_f64 * t1648 * t2643;
    let t7925 = t5493 * t2602;
    let t7927 = 16.0_f64 / 45.0_f64 * t639 * t7925;
    let t7932 = t5018 * t2631;
    let t7934 = 16.0_f64 / 45.0_f64 * t587 * t7932;
    let t7940 = t837 * t589;
    (t7907, t7915, t7919, t7927, t7934, t7940)
}
