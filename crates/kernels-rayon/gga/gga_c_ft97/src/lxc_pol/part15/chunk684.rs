//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 684/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk684(t20087: f64, t409: f64, t64: f64, t4474: f64, t938: f64, t8052: f64, t11160: f64, t15782: f64, t15793: f64, t15797: f64, t1599: f64, t1624: f64, t19977: f64, t19978: f64, t19983: f64, t19987: f64, t19995: f64, t19998: f64, t20004: f64, t20008: f64, t20012: f64, t20050: f64, t3076: f64, t3077: f64, t372: f64, t374: f64, t4491: f64, t534: f64, t7906: f64, t7914: f64, t8042: f64) -> (f64, f64) {
    let t20089 = t64 * t409 * t20087;
    let t20090 = t4474 * t938;
    let t20092 = t64 * t8052 * t20090;
    let t20097 = -0.32253953169881963531e-5_f64 * t372 * t534 * t19978 - 0.69764702839313376e-1_f64 * t8042 * t19983 - 0.69764702839313376e-2_f64 * t1624 * t19987 - 0.33776098467676728323e-5_f64 * t534 * t19977 * t1599 + 0.58097170218823199823e-3_f64 * t372 * t19995 - 0.58097170218823199822e-3_f64 * t1624 * t19998 - 0.279058811357253504e-2_f64 * t372 * t7914 * t19978 + 0.69764702839313376e-2_f64 * t372 * t20004 + 0.34882351419656688e-1_f64 * t1624 * t20008 + 0.34882351419656688e-1_f64 * t1624 * t20012 - 0.11619434043764639964e-3_f64 * t372 * t7906 * t19978 - 0.11627450473218896e-1_f64 * t372 * t374 * t20050 - 0.20279640676073749279e-3_f64 * t15797 * t11160 - 0.40559281352147498558e-4_f64 * t7906 * t19977 * t1599 + 0.20279640676073749279e-3_f64 * t15793 * t15782 - t20089 - 6.0_f64 * t20092 + 6.0_f64 * t3076 * t3077 * t4491;
    (t20090, t20097)
}
