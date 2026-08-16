//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1183/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1183(t13942: f64, t2080: f64, t3803: f64, t833: f64, t1178: f64, t371: f64, t3722: f64, t1177: f64, t3737: f64, t13830: f64, t14617: f64, t14657: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15288 = t2080 * t3803 * t13942;
    let t15289 = t15288 * t833;
    let t15296 = t371 * t1178 * t3722;
    let t15297 = t1177 * t15296;
    let t15309 = t371 * t1178 * t3737;
    let t15310 = t13830 * t15309;
    let t15312 = t14657 * t14617;
    (t15288, t15289, t15296, t15297, t15309, t15310, t15312)
}
