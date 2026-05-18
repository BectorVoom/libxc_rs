//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1081/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1081<F: Float>(t558: F, t9565: F, t1562: F, t9343: F, t1734: F, t2228: F, t1550: F, t2211: F, t42954: F, t44713: F, t45453: F, t45458: F, t45463: F, t45466: F, t45469: F, t45473: F, t45477: F, t45482: F, t45484: F, t45486: F, t45488: F, t6415: F, t6418: F, t699: F, t739: F, t884: F, t903: F) -> (F, F, F) {
    let t48482 = t9565 * t558;
    let t48485 = t1562 * t9343;
    let t48489 = t2228 * t1734;
    let t48498 = F::new(0.1702583995731913576e-4) * t45453 + F::new(0.1702583995731913576e-4) * t45458 + F::new(0.1702583995731913576e-4) * t45463 + t42954 - F::new(0.11974241701863808564e0) * t1550 * t699 * t6415 + F::new(0.17961362552795712846e0) * t903 * t699 * t6418 + F::new(0.11974241701863808564e0) * t739 * t2211 * t44713 + F::new(0.11974241701863808564e0) * t884 * t48482 - F::new(0.4726e1) * t48485 + F::new(0.40911992481368012596e-1) * t45466 + F::new(0.40911992481368012596e-1) * t45469 - F::new(0.59871208509319042821e-1) * t739 * t48489 - F::new(0.49658699875514145967e-4) * t45473 - F::new(0.85129199786595678799e-5) * t45477 + F::new(0.1064114997332445985e-4) * t45482 + F::new(0.1702583995731913576e-4) * t45484 - F::new(0.49658699875514145967e-4) * t45486 - F::new(0.39726959900411316773e-4) * t45488;
    (t48482, t48489, t48498)
}
