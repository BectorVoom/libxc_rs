//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1193/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1193(t14092: f64, t6706: f64, t14028: f64, t2339: f64, t14022: f64, t885: f64, t2149: f64, t854: f64, t6238: f64, t899: f64, t922: f64, t2268: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51282 = t14092 * t6706;
    let t51285 = t14028 * t2339;
    let t51291 = t14022 * t885;
    let t51292 = t51291 * t2149;
    let t51293 = t854 * t51292;
    let t51301 = t899 * t6238 * t922;
    let t51302 = t51301 * t2268;
    (t51282, t51285, t51291, t51292, t51293, t51302)
}
