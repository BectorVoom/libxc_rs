//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1076/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1076(t35772: f64, t37848: f64, t37849: f64, t37850: f64, t4041: f64, t40518: f64, t40529: f64, t40533: f64, t40537: f64, t40541: f64, t40544: f64, t40547: f64, t40554: f64, t40556: f64, t40558: f64, t4965: f64, t623: f64, t8160: f64, t9624: f64, t9627: f64) -> f64 {
    let t43465 = 0.35922725105591425692e0_f64 * t40518 - 0.23948483403727617128e0_f64 * t4041 * t9624 - 0.23948483403727617128e0_f64 * t4965 * t9627 - 0.5107751987195740728e-4_f64 * t40529 - 0.19957069503106347607e-1_f64 * t623 * t8160 - 0.7661627980793611092e-4_f64 * t40533 + 0.10215503974391481456e-3_f64 * t40537 + 0.2553875993597870364e-4_f64 * t40541 - 0.1440846329149835838e-2_f64 * t40544 - 0.72042316457491791901e-3_f64 * t40547 - 0.60975299583150056624e-3_f64 * t35772 - t37848 - t37849 + t37850 + 0.1064114997332445985e-4_f64 * t40554 - 0.2553875993597870364e-4_f64 * t40556 - 0.49658699875514145966e-4_f64 * t40558;
    t43465
}
