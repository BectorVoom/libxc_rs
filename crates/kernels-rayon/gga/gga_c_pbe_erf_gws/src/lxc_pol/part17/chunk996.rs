//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 996/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk996(t3106: f64, t6472: f64, t8782: f64, t860: f64, t3116: f64, t6707: f64, t1105: f64, t2182: f64) -> (f64, f64, f64) {
    let t8933 = t6472 * t3106;
    let t8934 = t8782 * t8933;
    let t8936 = t8934 * t860 / 96.0_f64;
    let t8938 = t3116 * t6707 / 96.0_f64;
    let t8939 = t1105 * t2182;
    (t8936, t8938, t8939)
}
