//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1178/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1178(t15204: f64, t3983: f64, t2503: f64, t4127: f64, t3863: f64, t4039: f64, t3788: f64, t4023: f64, t14015: f64, t3754: f64, t3749: f64, t3783: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15205 = t3983 * t15204;
    let t15216 = t4127 * t2503;
    let t15218 = t4039 * t3863;
    let t15220 = t3788 * t4023;
    let t15222 = t14015 * t3754;
    let t15224 = t4039 * t3749;
    let t15226 = t3783 * t4023;
    (t15205, t15216, t15218, t15220, t15222, t15224, t15226)
}
