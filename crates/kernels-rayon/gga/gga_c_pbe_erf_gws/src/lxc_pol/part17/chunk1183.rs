//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1183/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1183(t27047: f64, t3067: f64, t4016: f64, t814: f64, t13784: f64, t13808: f64, t1192: f64, t19631: f64, t829: f64, t830: f64, t2271: f64, t332: f64) -> (f64, f64, f64, f64) {
    let t50924 = t27047 * t3067 * t4016 * t814;
    let t50927 = t13808 * t13784;
    let t50930 = t19631 * t1192;
    let t50932 = t829 * t830 * t50930;
    let t50935 = t2271 * t332;
    (t50924, t50927, t50932, t50935)
}
