//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 347/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk347(t1033: f64, t221: f64, t643: f64, t954: f64, t642: f64, t639: f64, t1027: f64, t657: f64, t1029: f64, t25: f64, t651: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1035 = 2.0_f64 / 15.0_f64 * t1033 * t221;
    let t1036 = t643 * t954;
    let t1037 = t642 * t1036;
    let t1039 = 4.0_f64 / 45.0_f64 * t639 * t1037;
    let t1041 = t657 * t1027;
    let t1044 = -t651 - 0.35991666666666666667e-1_f64 * t1029 - t655 - 0.66666666666666666667e-2_f64 * t25 * t1041;
    (t1035, t1036, t1037, t1039, t1041, t1044)
}
