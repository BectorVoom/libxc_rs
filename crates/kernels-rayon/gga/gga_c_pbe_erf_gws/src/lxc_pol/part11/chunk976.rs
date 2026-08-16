//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 976/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk976(t1663: f64, t3443: f64, t1672: f64, t3563: f64, t616: f64, t1251: f64, t3550: f64, t3544: f64, t3547: f64, t2790: f64, t7956: f64, t1764: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32260 = t3443 * t1663;
    let t32279 = t616 * t1672 * t3563;
    let t32373 = t1251 * t3550;
    let t32375 = t1251 * t3544;
    let t32405 = t1251 * t3547;
    let t32523 = t2790 * t7956;
    let t32629 = t3443 * t1764;
    (t32260, t32279, t32373, t32375, t32405, t32523, t32629)
}
