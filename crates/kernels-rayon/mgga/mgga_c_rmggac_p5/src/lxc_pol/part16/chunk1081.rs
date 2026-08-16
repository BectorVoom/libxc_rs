//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1081/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1081(t558: f64, t9565: f64, t1562: f64, t9343: f64, t1734: f64, t2228: f64, t1550: f64, t2211: f64, t42954: f64, t44713: f64, t45453: f64, t45458: f64, t45463: f64, t45466: f64, t45469: f64, t45473: f64, t45477: f64, t45482: f64, t45484: f64, t45486: f64, t45488: f64, t6415: f64, t6418: f64, t699: f64, t739: f64, t884: f64, t903: f64) -> (f64, f64, f64) {
    let t48482 = t9565 * t558;
    let t48485 = t1562 * t9343;
    let t48489 = t2228 * t1734;
    let t48498 = 0.1702583995731913576e-4_f64 * t45453 + 0.1702583995731913576e-4_f64 * t45458 + 0.1702583995731913576e-4_f64 * t45463 + t42954 - 0.11974241701863808564e0_f64 * t1550 * t699 * t6415 + 0.17961362552795712846e0_f64 * t903 * t699 * t6418 + 0.11974241701863808564e0_f64 * t739 * t2211 * t44713 + 0.11974241701863808564e0_f64 * t884 * t48482 - 0.4726e1_f64 * t48485 + 0.40911992481368012596e-1_f64 * t45466 + 0.40911992481368012596e-1_f64 * t45469 - 0.59871208509319042821e-1_f64 * t739 * t48489 - 0.49658699875514145967e-4_f64 * t45473 - 0.85129199786595678799e-5_f64 * t45477 + 0.1064114997332445985e-4_f64 * t45482 + 0.1702583995731913576e-4_f64 * t45484 - 0.49658699875514145967e-4_f64 * t45486 - 0.39726959900411316773e-4_f64 * t45488;
    (t48482, t48489, t48498)
}
