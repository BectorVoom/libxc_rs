//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 756/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk756<F: Float>(t1109: F, t13399: F, t2378: F, t2380: F, t200: F, t2417: F, t679: F, t1609: F, t223: F, t9542: F, t4952: F, t6783: F, t2455: F, t3780: F, t1127: F, t2427: F) -> (F, F, F, F, F, F, F) {
    let t13400 = t13399 * t1109;
    let t13401 = t2378 * t2380;
    let t13402 = t13401 * t200;
    let t13406 = t679 * t2417;
    let t13407 = t13406 * t200;
    let t13411 = t1609 * t2378;
    let t13412 = t9542 * t223;
    let t13413 = t13411 * t13412;
    let t13414 = t6783 * t4952;
    let t13417 = t3780 * t2455;
    let t13421 = t2427 * t1127;
    (t13400, t13402, t13407, t13413, t13414, t13417, t13421)
}
