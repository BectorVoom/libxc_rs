//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1138/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1138(t1184: f64, t3195: f64, t3295: f64, t4039: f64, t4022: f64, t863: f64, t6523: f64, t8867: f64, t1150: f64, t14028: f64, t14042: f64, t14047: f64, t14529: f64, t14531: f64, t14533: f64, t14536: f64, t14539: f64) -> (f64, f64, f64) {
    let t14542 = t1184 * t3195;
    let t14544 = t4039 * t3295;
    let t14547 = t863 * t4022;
    let t14548 = t6523 * t8867;
    let t14549 = t14547 * t14548;
    let t14551 = t14028 * t1150;
    let t14553 = -t14529 / 768.0_f64 - t14531 / 192.0_f64 - t14533 / 48.0_f64 - t14536 / 48.0_f64 - t14539 / 96.0_f64 + 7.0_f64 / 144.0_f64 * t14042 - t14542 / 48.0_f64 + t14544 / 768.0_f64 + 7.0_f64 / 288.0_f64 * t14047 + t14549 / 16.0_f64 - 7.0_f64 / 1152.0_f64 * t14551;
    (t14547, t14548, t14553)
}
