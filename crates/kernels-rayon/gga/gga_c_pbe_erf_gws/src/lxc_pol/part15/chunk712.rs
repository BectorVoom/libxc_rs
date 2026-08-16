//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 712/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk712(t2409: f64, t3067: f64, t4164: f64, t1125: f64, t4023: f64, t3132: f64, t3139: f64, t4028: f64, t1140: f64, t1184: f64, t1150: f64, t4039: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4166 = t2409 * t3067 * t4164;
    let t4169 = t1125 * t4023;
    let t4171 = t3139 * t3132;
    let t4172 = t4028 * t4171;
    let t4174 = t1184 * t1140;
    let t4176 = t4039 * t1150;
    (t4166, t4169, t4171, t4172, t4174, t4176)
}
