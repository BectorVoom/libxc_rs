//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 558/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk558<F: Float>(t43: F, t2413: F, t2827: F, t1891: F, t1210: F, t1214: F, t429: F, t529: F, t496: F, t492: F, t490: F, t149: F, t209: F, t371: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t2828 = t2413 + t2827;
    let t2832 = piecewise3(t44, 0.0, t1891);
    let t2835 = t1210 * t1214;
    let t2837 = t529 * t429;
    let t2838 = t2837 * t496;
    let t2839 = t492 * t2838;
    let t2841 = t490 * t2839 / 9.0;
    let t2843 = t209 * t149 * t371;
    (t2828, t2832, t2835, t2837, t2839, t2841, t2843)
}
