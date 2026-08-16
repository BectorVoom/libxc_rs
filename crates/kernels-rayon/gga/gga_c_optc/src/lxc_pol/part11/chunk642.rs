//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 642/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk642(t1583: f64, t5255: f64, t1582: f64, t1492: f64, t4305: f64, t1220: f64, t1579: f64, t1588: f64, t277: f64, t2841: f64, t2911: f64, t4216: f64, t4230: f64, t4297: f64, t4536: f64, t5087: f64, t5098: f64, t5103: f64, t5167: f64, t5226: f64, t5229: f64, t5233: f64, t5243: f64, t5246: f64, t5250: f64, t95: f64) -> (f64, f64, f64, f64) {
    let t5256 = t1583 * t5255;
    let t5257 = t1582 * t5256;
    let t5261 = 0.11696446794910408142e1_f64 * t4305 * t1492;
    let t5262 = -0.25844881434903430496e-2_f64 * t95 * t277 * t5087 * t2911 + t4536 * t1579 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t4230 * t1579 + t1220 * t5098 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t1220 * t5103 + t5226 + 100.0_f64 / 81.0_f64 * t4216 + 100.0_f64 / 81.0_f64 * t4297 * t5229 - t1220 * t5233 / 3.0_f64 + 20000.0_f64 / 81.0_f64 * t5243 * t5246 + 100.0_f64 / 27.0_f64 * t5250 * t1588 - 50.0_f64 / 3.0_f64 * t5257 * t1588 - t2841 - t5167 - t5261;
    (t5256, t5257, t5261, t5262)
}
