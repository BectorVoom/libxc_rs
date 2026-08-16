//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 987/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk987(t3133: f64, t6183: f64, t2134: f64, t1133: f64, t874: f64, t2171: f64, t4386: f64, t2168: f64, t6185: f64, t3179: f64, t6331: f64, t2146: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8824 = t6183 * t3133;
    let t8826 = 7.0_f64 / 144.0_f64 * t2134 * t8824;
    let t8827 = t1133 * t874;
    let t8828 = t8827 * t2171;
    let t8829 = t4386 * t8828;
    let t8831 = t2168 * t8829 / 24.0_f64;
    let t8832 = 7.0_f64 / 144.0_f64 * t6185;
    let t8833 = t6331 * t3179;
    let t8835 = 7.0_f64 / 72.0_f64 * t2146 * t8833;
    (t8826, t8827, t8828, t8831, t8832, t8835)
}
