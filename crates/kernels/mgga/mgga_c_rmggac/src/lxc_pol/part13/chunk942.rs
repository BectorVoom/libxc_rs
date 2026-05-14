//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 942/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk942<F: Float>(t35772: F, t37848: F, t37849: F, t37850: F, t4041: F, t40518: F, t40529: F, t40533: F, t40537: F, t40541: F, t40544: F, t40547: F, t40554: F, t40556: F, t40558: F, t4965: F, t623: F, t8160: F, t9624: F, t9627: F) -> (F,) {
    let t43465 = 0.35922725105591425692e0 * t40518 - 0.23948483403727617128e0 * t4041 * t9624 - 0.23948483403727617128e0 * t4965 * t9627 - 0.5107751987195740728e-4 * t40529 - 0.19957069503106347607e-1 * t623 * t8160 - 0.7661627980793611092e-4 * t40533 + 0.10215503974391481456e-3 * t40537 + 0.2553875993597870364e-4 * t40541 - 0.1440846329149835838e-2 * t40544 - 0.72042316457491791901e-3 * t40547 - 0.60975299583150056624e-3 * t35772 - t37848 - t37849 + t37850 + 0.1064114997332445985e-4 * t40554 - 0.2553875993597870364e-4 * t40556 - 0.49658699875514145966e-4 * t40558;
    (t43465,)
}
