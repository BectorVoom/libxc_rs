//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 793/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk793(t159: f64, t285: f64, t5984: f64, t169: f64, t274: f64, t301: f64, t922: f64, t1477: f64, t535: f64, t551: f64, t1480: f64, t1371: f64, t1478: f64) -> (f64, f64, f64, f64) {
    let t6032 = 0.67153358174671991426e-2_f64 * t5984 * t159 * t285;
    let t6036 = 0.92478548207158653218e0_f64 * t169 * t922 * t274 * t301;
    let t6037 = t1477 * t535;
    let t6038 = t6037 * t551;
    let t6039 = t6038 * t1480;
    let t6041 = t1478 * t1371;
    (t6032, t6036, t6039, t6041)
}
