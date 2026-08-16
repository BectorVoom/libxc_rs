//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 667/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk667(t1188: f64, t1570: f64, t1575: f64, t277: f64, t4276: f64, t4278: f64, t4540: f64, t490: f64, t498: f64, t5169: f64, t5173: f64, t5189: f64, t5192: f64, t5266: f64, t5270: f64, t5306: f64, t5310: f64, t5434: f64, t5441: f64, t5474: f64, t95: f64) -> f64 {
    let t5478 = t5266 - t5270 - 8.0_f64 / 9.0_f64 * t4276 + t4278 / 3.0_f64 + 0.25844881434903430496e-2_f64 * t95 * t277 * t5434 * t1188 - t5310 + 44.0_f64 / 9.0_f64 * t490 * t5441 - 8.0_f64 / 3.0_f64 * t1570 * t1575 + t5474 * t498 / 2.0_f64 + t4540 / 9.0_f64 + t5306 + t5169 - t5173 + t5189 + t5192;
    t5478
}
