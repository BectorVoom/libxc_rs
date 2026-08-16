//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 832/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk832(t6964: f64, t87: f64, t40: f64, t4: f64, t959: f64, t1448: f64, t2551: f64, t735: f64, t34: f64, t92: f64, t93: f64, t108: f64, t1403: f64, t1407: f64, t1413: f64, t1416: f64, t2538: f64, t2541: f64, t2544: f64, t2547: f64, t39: f64, t532: f64, t6937: f64, t6952: f64, t726: f64, t728: f64, t964: f64, t965: f64) -> (f64, f64, f64, f64) {
    let t6965 = t6964 * t87;
    let t6966 = t40 * t6965;
    let t6967 = t959 * t4;
    let t6968 = t6967 * t1448;
    let t6969 = 0.10843580882781524214e-1_f64 * t6968;
    let t6971 = 4.0_f64 / 45.0_f64 * t2551 * t735;
    let t6974 = t92 * t34;
    let t6985 = t93 * t34;
    let t6995 = (40.0_f64 / 27.0_f64 * t964 * t1403 + 80.0_f64 / 9.0_f64 * t6974 * t6937 + 20.0_f64 / 9.0_f64 * t2538 * t1407 + 8.0_f64 / 3.0_f64 * t726 * t532 - 8.0_f64 * t2541 * t39 + 40.0_f64 / 27.0_f64 * t965 * t1413 - 80.0_f64 / 9.0_f64 * t6985 * t6952 + 20.0_f64 / 9.0_f64 * t2544 * t1416 - 8.0_f64 / 3.0_f64 * t728 * t532 + 8.0_f64 * t2547 * t39) * t108;
    (t6966, t6969, t6971, t6995)
}
