//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1010/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1010(t9043: f64, t9044: f64, t9046: f64, t9048: f64, t3028: f64, t369: f64, t1109: f64, t931: f64, t1130: f64, t2182: f64, t3162: f64, t810: f64) -> (f64, f64, f64, f64, f64) {
    let t9050 = t9043 + t9044 + t9046 + t9048;
    let t9053 = t3028 * t369;
    let t9056 = t1109 * t931;
    let t9067 = t1130 * t2182;
    let t9070 = t3162 * t810;
    (t9050, t9053, t9056, t9067, t9070)
}
