//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 862/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk862(t226: f64, t7: f64, t7236: f64, t7271: f64, t1735: f64, t7632: f64, t1750: f64, t1795: f64, t1775: f64, t1868: f64, t1680: f64, t1872: f64, t7839: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t16595 = 4.0_f64 / 3.0_f64 * t226 * (-0.42777777777777777777e1_f64 * t7271 + 220.0_f64 / 81.0_f64 * t7236) * pi * t7;
    let t16597 = 16.0_f64 / 5.0_f64 * t7632 * t1735;
    let t16599 = 8.0_f64 / 5.0_f64 * t1750 * t1795;
    let t16601 = 4.0_f64 / 5.0_f64 * t1775 * t1868;
    let t16603 = 8.0_f64 / 5.0_f64 * t1680 * t1868;
    let t16605 = 16.0_f64 / 5.0_f64 * t7839 * t1872;
    (t16595, t16597, t16599, t16601, t16603, t16605)
}
