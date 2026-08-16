//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1190/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1190(t1178: f64, t371: f64, t3737: f64, t13830: f64, t14617: f64, t14657: f64, t2409: f64, t9897: f64, t3965: f64, t9818: f64, t14121: f64, t1105: f64, t4182: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15309 = t371 * t1178 * t3737;
    let t15310 = t13830 * t15309;
    let t15312 = t14657 * t14617;
    let t15314 = t2409 * t9897;
    let t15315 = t3965 * t15314;
    let t15317 = t2409 * t9818;
    let t15318 = t14121 * t15317;
    let t15320 = t4182 * t1105;
    (t15309, t15310, t15312, t15314, t15315, t15317, t15318, t15320)
}
