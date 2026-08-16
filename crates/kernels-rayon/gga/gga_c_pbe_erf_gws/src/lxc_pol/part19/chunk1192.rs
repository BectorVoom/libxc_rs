//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1192/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1192(t1205: f64, t3886: f64, t2409: f64, t3067: f64, t1144: f64, t338: f64, t4228: f64, t1109: f64, t1206: f64, t353: f64, t859: f64, t3717: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15526 = t1205 * t3886;
    let t15528 = t2409 * t3067 * t15526;
    let t15532 = t338 * t1144 * t4228;
    let t15535 = t1206 * t1109;
    let t15536 = t353 * t15535;
    let t15537 = t859 * t15536;
    let t15543 = t1205 * t3717;
    (t15526, t15528, t15532, t15535, t15536, t15537, t15543)
}
