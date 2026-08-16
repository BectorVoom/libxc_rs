//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1056/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1056(t1125: f64, t9375: f64, t3449: f64, t3565: f64, t3832: f64, t7056: f64, t11046: f64, t3268: f64, t3265: f64, t3622: f64, t11614: f64, t11617: f64, t11621: f64, t11623: f64, t11627: f64, t11630: f64, t11634: f64, t11638: f64, t11641: f64, t11644: f64, t11649: f64, t11651: f64, t11653: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12155 = t9375 * t1125;
    let t12156 = t3565 * t3449;
    let t12158 = 2.0_f64 * t7056 * t3832;
    let t12161 = 2.0_f64 * t11046 * t3268;
    let t12162 = t3265 * t3622;
    let t12176 = -0.16414765573575218917e-4_f64 * t11614 - 0.16414765573575218917e-4_f64 * t11617 + 0.23485962392041415794e-5_f64 * t11621 - 0.16146599144528473358e-4_f64 * t11623 + 0.23485962392041415794e-4_f64 * t11627 + 0.14678726495025884871e-5_f64 * t11630 + 0.86995919027186744337e-7_f64 * t11634 + 0.14678726495025884871e-5_f64 * t11638 + 0.23485962392041415794e-4_f64 * t11641 - 0.34197428278281706076e-6_f64 * t11644 - 0.99742499144988309388e-7_f64 * t11649 + 0.30777685450453535468e-5_f64 * t11651 + 0.93943849568165663176e-4_f64 * t11653;
    (t12155, t12156, t12158, t12161, t12162, t12176)
}
