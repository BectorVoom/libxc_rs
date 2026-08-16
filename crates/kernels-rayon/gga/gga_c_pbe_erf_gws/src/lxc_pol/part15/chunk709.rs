//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 709/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk709(t3959: f64, t4130: f64, t1146: f64, t1173: f64, t2409: f64, t3045: f64, t3965: f64, t1149: f64, t3975: f64, t3972: f64, t1118: f64, t1178: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4131 = t3959 * t4130;
    let t4133 = t1173 * t1146;
    let t4135 = t2409 * t3045;
    let t4136 = t3965 * t4135;
    let t4138 = t3975 * t1149;
    let t4139 = t3972 * t4138;
    let t4141 = t1178 * t1118;
    (t4131, t4133, t4135, t4136, t4138, t4139, t4141)
}
