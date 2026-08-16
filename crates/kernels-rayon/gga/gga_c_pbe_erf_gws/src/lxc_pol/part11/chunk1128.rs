//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1128/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1128(t12355: f64, t2678: f64, t10534: f64, t3354: f64, t3465: f64, t2672: f64, t11: f64, t1691: f64, t47969: f64, t625: f64, t5089: f64, t1714: f64, t25: f64, t40962: f64, t40989: f64, t5061: f64, t657: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47975 = t2678 * t12355;
    let t47979 = t10534 * t3354;
    let t47983 = t3465 * t3354;
    let t47987 = t2672 * t12355;
    let t47994 = t11 * t1691 * t47969;
    let t47997 = t11 * t1691 * t47975;
    let t48000 = t11 * t625 * t47983;
    let t48003 = t11 * t625 * t47987;
    let t48006 = t11 * t5089 * t47979;
    let t48008 = -0.88888888888888888888e-2_f64 * t25 * t1714 * t47975 - 0.17777777777777777778e-1_f64 * t25 * t5061 * t47979 - 0.24e0_f64 * t25 * t657 * t47983 + 0.53333333333333333332e-1_f64 * t25 * t657 * t47987 + 0.95977777777777777777e-1_f64 * t40962 - 0.28793333333333333333e0_f64 * t40989 + 0.86380000000000000002e0_f64 * t47994 - 0.9597777777777777778e-1_f64 * t47997 - 0.12957e1_f64 * t48000 + 0.28793333333333333333e0_f64 * t48003 - 0.23994444444444444446e0_f64 * t48006;
    (t47975, t47979, t47983, t47987, t47994, t47997, t48000, t48003, t48006, t48008)
}
