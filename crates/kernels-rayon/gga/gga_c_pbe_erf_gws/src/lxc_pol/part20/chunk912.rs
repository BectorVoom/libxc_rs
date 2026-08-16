//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 912/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk912(t10037: f64, t525: f64, t2036: f64, t3641: f64, t3619: f64, t5660: f64, t142: f64, t2900: f64, t2031: f64, t4561: f64, t7906: f64, t7907: f64) -> (f64, f64, f64, f64, f64) {
    let t10186 = t525 * t10037;
    let t10189 = t3641 * t2036;
    let t10194 = t5660 * t3619;
    let t10196 = t142 * t2900;
    let t10197 = t2031 * t10196;
    let t10201 = -t7906 + t7907 + t4561;
    (t10186, t10189, t10194, t10197, t10201)
}
