//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1220/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1220(t876: f64, t9246: f64, t1185: f64, t326: f64, t346: f64, t6045: f64, t2250: f64, t51213: f64, t14006: f64, t6684: f64, t816: f64, t837: f64) -> (f64, f64, f64, f64, f64) {
    let t51430 = t9246 * t876;
    let t51458 = t326 * t346 * t6045 * t1185;
    let t51459 = 455.0_f64 / 1296.0_f64 * t51458;
    let t51465 = t2250 * t51213;
    let t51470 = t6684 * t14006;
    let t51502 = t816 * t837;
    (t51430, t51459, t51465, t51470, t51502)
}
