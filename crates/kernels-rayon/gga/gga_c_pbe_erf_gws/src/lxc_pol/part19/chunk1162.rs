//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1162/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1162(t14655: f64, t4218: f64, t9270: f64, t14295: f64, t14302: f64, t14305: f64, t14634: f64, t14640: f64, t14649: f64, t14658: f64, t14945: f64, t14949: f64, t14954: f64, t14959: f64, t2408: f64, t3066: f64) -> (f64, f64) {
    let t14962 = 7.0_f64 / 576.0_f64 * t14655;
    let t14964 = t9270 * t4218;
    let t14967 = t14634 / 384.0_f64 + 5.0_f64 / 384.0_f64 * t14640 + t3066 * t14945 / 48.0_f64 + t3066 * t14949 / 48.0_f64 + t2408 * t14954 / 48.0_f64 - t14649 / 48.0_f64 - t2408 * t14959 / 24.0_f64 + t14295 + t14962 - t14302 - t14658 / 48.0_f64 - 7.0_f64 / 144.0_f64 * t14964 - 7.0_f64 / 144.0_f64 * t14305;
    (t14964, t14967)
}
