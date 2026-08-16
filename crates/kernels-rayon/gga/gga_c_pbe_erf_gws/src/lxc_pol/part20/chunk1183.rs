//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1183/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1183(t3827: f64, t4043: f64, t3820: f64, t4028: f64, t1125: f64, t14535: f64, t14092: f64, t3792: f64, t14538: f64, t3857: f64, t14011: f64, t3816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15234 = t4043 * t3827;
    let t15236 = t4028 * t3820;
    let t15238 = t1125 * t14535;
    let t15240 = t14092 * t3792;
    let t15241 = t14538 * t15240;
    let t15243 = t4043 * t3857;
    let t15245 = t14011 * t3816;
    (t15234, t15236, t15238, t15240, t15241, t15243, t15245)
}
