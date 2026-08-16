//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1317/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1317(t11701: f64, t14015: f64, t12088: f64, t14007: f64, t12050: f64, t11656: f64, t11782: f64, t14069: f64, t11502: f64, t11829: f64, t2407: f64, t3116: f64, t35207: f64, t858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t56861 = t14015 * t11701;
    let t56863 = t14007 * t12088;
    let t56865 = t14007 * t12050;
    let t56867 = t14007 * t11656;
    let t56869 = t11782 * t14069;
    let t56871 = t14007 * t11502;
    let t56873 = t14007 * t11829;
    let t56877 = t3116 * t2407 * t858 * t35207;
    (t56861, t56863, t56865, t56867, t56869, t56871, t56873, t56877)
}
