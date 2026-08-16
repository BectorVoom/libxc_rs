//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 359/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk359(t1188: f64, t1216: f64, t1220: f64, t1448: f64, t1462: f64, t1488: f64, t1490: f64, t1494: f64, t1554: f64, t1570: f64, t1575: f64, t1579: f64, t1585: f64, t1588: f64, t277: f64, t490: f64, t498: f64, t95: f64) -> f64 {
    let t1591 = -t1448 + t1462 + t1488 + t1490 - t1494 + 0.25844881434903430496e-2_f64 * t95 * t277 * t1554 * t1188 + t1570 * t498 / 2.0_f64 - 4.0_f64 / 3.0_f64 * t490 * t1575 + t1216 + t1220 * t1579 / 6.0_f64 + 50.0_f64 / 27.0_f64 * t1585 * t1588;
    t1591
}
