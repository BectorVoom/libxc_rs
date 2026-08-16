//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1034/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1034(t43: f64, t1285: f64, t1274: f64, t404: f64, t1399: f64, t4788: f64, t260: f64, t1402: f64, t1403: f64, t1407: f64, t16669: f64, t16679: f64, t16746: f64, t4360: f64, t47: f64, t4757: f64, t4760: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t18664 = t1285 * t1285;
    let t18667 = 6.0_f64 * t1274 * t18664 * t404;
    let t18668 = t1399 * t4788;
    let t18669 = 0.23392893589820816284e1_f64 * t18668;
    let t18670 = 1.0_f64 / t260;
    let t18683 = piecewise3(t44, 0.0_f64, 40.0_f64 / 81.0_f64 * t18670 * t16669 - 16.0_f64 / 9.0_f64 * t4757 * t1403 * t1407 + 4.0_f64 / 3.0_f64 * t1402 * t16679 + 16.0_f64 / 9.0_f64 * t4760 * t4360 + 4.0_f64 / 3.0_f64 * t47 * t16746);
    (t18664, t18667, t18669, t18683)
}
