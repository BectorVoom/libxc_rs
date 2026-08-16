//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2807/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2807<F: Float>(t51587: F, t10861: F, t10943: F, t14502: F, t14535: F, t14961: F, t2754: F, t40894: F, t4504: F, t4514: F, t51553: F, t51561: F, t51565: F, t51572: F, t51576: F, t51578: F, t820: F) -> F {
    let t51588 = F::cast_from(0.39029762157531132076e-1_f64) * t51587;
    let t51589 = -F::cast_from(0.26019841438354088051e-2_f64) * t51553 + t51561 + t51565 + F::cast_from(0.54878743191129263322e-2_f64) * t40894 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t14961 * t10861 + F::cast_from(0.98781737744032673976e-1_f64) * t51572 - F::cast_from(0.98781737744032673976e-1_f64) * t51576 - F::cast_from(0.11044544084478153697e-3_f64) * t51578 + F::cast_from(0.39512695097613069591e1_f64) * t4504 * t14535 * t10943 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t14502 * t2754 - t51588;
    t51589
}
