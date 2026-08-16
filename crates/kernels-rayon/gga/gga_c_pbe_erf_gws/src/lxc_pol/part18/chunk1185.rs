//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1185/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1185(t11776: f64, t3139: f64, t4028: f64, t3871: f64, t4049: f64, t3875: f64, t11785: f64, t14101: f64, t1184: f64, t3799: f64, t3867: f64, t3805: f64, t4023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15248 = t3139 * t11776;
    let t15249 = t4028 * t15248;
    let t15251 = t4049 * t3871;
    let t15253 = t4049 * t3875;
    let t15255 = t3139 * t11785;
    let t15256 = t14101 * t15255;
    let t15258 = t1184 * t3799;
    let t15260 = t1184 * t3867;
    let t15262 = t3805 * t4023;
    (t15248, t15249, t15251, t15253, t15255, t15256, t15258, t15260, t15262)
}
