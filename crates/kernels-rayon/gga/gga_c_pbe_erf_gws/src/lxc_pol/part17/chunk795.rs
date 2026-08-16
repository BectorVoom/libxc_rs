//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 795/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk795(t1477: f64, t535: f64, t551: f64, t1480: f64, t1371: f64, t1478: f64, t412: f64, t8: f64, t147: f64, t1473: f64, t755: f64, t759: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6037 = t1477 * t535;
    let t6038 = t6037 * t551;
    let t6039 = t6038 * t1480;
    let t6041 = t1478 * t1371;
    let t6043 = 0.54655730795145295329e-4_f64 * t6041 * t1480;
    let t6045 = 1.0_f64 / t8 / t412;
    let t6046 = t6045 * t147;
    let t6047 = t6046 * t551;
    let t6049 = 0.16396719238543588599e-3_f64 * t6047 * t1480;
    let t6050 = t1473 * t755;
    let t6053 = 0.15965645347006145458e0_f64 * t1473 * t759;
    (t6039, t6043, t6045, t6049, t6050, t6053)
}
