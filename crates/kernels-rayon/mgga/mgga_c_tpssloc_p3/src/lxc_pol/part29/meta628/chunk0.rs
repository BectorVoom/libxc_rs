//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2072/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2072(t1235: f64, t225: f64, t461: f64, t24574: f64, t24626: f64, t24617: f64, t11553: f64, t2121: f64, t2123: f64, t2122: f64, t85628: f64, t24884: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86415 = t461 * t1235 * t225;
    let t86424 = t24574 * t24626;
    let t86426 = t24574 * t24617;
    let t86451 = 0.30461741978670859935e-2_f64 * t2121 * t11553 * t2123;
    let t86452 = t2122 * t85628;
    let t86456 = t24574 * t24884;
    (t86415, t86424, t86426, t86451, t86452, t86456)
}
