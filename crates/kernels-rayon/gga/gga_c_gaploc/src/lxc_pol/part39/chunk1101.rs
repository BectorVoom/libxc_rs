//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1101/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1101(t42511: f64, t12270: f64, t2592: f64, t42517: f64, t13765: f64, t4342: f64, t1382: f64, t2497: f64, t3718: f64, t13914: f64, t1960: f64, t42513: f64, t44207: f64, t44208: f64, t44211: f64, t44215: f64, t44217: f64, t841: f64) -> (f64, f64, f64, f64, f64) {
    let t47110 = 2.0_f64 * t42511;
    let t47112 = t2592 * t12270;
    let t47113 = 2.0_f64 * t42517;
    let t47114 = t4342 * t13765;
    let t47115 = 2.0_f64 * t47114;
    let t47120 = t1382 * t3718 * t2497;
    let t47121 = 2.0_f64 * t47120;
    let t47124 = 2.0_f64 * t13914 * t1960 * t841 + t42513 - t44207 - t44208 + 2.0_f64 * t44211 + 2.0_f64 * t44215 + 2.0_f64 * t44217 - t47110 - t47112 - t47113 - t47115 - t47121;
    (t47110, t47113, t47115, t47121, t47124)
}
