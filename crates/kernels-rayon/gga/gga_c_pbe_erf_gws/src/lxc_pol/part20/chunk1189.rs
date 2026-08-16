//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1189/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1189(t13942: f64, t2080: f64, t3803: f64, t833: f64, t1144: f64, t338: f64, t4183: f64, t1178: f64, t371: f64, t3722: f64, t1177: f64, t1193: f64, t3907: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15288 = t2080 * t3803 * t13942;
    let t15289 = t15288 * t833;
    let t15292 = t338 * t1144 * t4183;
    let t15296 = t371 * t1178 * t3722;
    let t15297 = t1177 * t15296;
    let t15300 = t338 * t3907 * t1193;
    (t15288, t15289, t15292, t15296, t15297, t15300)
}
