//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1244/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1244(t193: f64, t2378: f64, t262: f64, t39658: f64, t40977: f64, t41270: f64, t41273: f64, t41275: f64, t41278: f64, t41281: f64, t41283: f64, t41286: f64, t41289: f64, t41292: f64, t41296: f64, t4314: f64, t776: f64, t868: f64, t870: f64, t9458: f64, t9516: f64) -> f64 {
    let t41603 = 24.0_f64 * t193 * t868 * t870 * t9458 + 24.0_f64 * t262 * t4314 * t776 * t9516 + 18.0_f64 * t193 * t2378 * t40977 - t39658 + t41270 + t41273 + t41275 + t41278 + t41281 + t41283 + t41286 + t41289 + t41292 + t41296;
    t41603
}
