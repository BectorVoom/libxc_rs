//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 341/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk341(t1000: f64, t571: f64, t11: f64, t570: f64, t173: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t1001 = t571 * t1000;
    let t1002 = t11 * t1001;
    let t1004 = t570 + 0.18891666666666666667e-2_f64 * t1002;
    let t1005 = t173 * t1004;
    let t1006 = t1005 * t184;
    (t1001, t1002, t1004, t1005, t1006)
}
