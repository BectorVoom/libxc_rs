//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1179/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1179(t14570: f64, t3123: f64, t14007: f64, t3759: f64, t14035: f64, t3837: f64, t3827: f64, t4043: f64, t3820: f64, t4028: f64, t1125: f64, t14535: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15228 = t3123 * t14570;
    let t15230 = t14007 * t3759;
    let t15232 = t14035 * t3837;
    let t15234 = t4043 * t3827;
    let t15236 = t4028 * t3820;
    let t15238 = t1125 * t14535;
    (t15228, t15230, t15232, t15234, t15236, t15238)
}
