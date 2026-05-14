//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 972/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk972<F: Float>(t1535: F, t2880: F, t2876: F, t1539: F, t2869: F, t1145: F, t1530: F, t2884: F) -> (F, F, F, F, F, F, F) {
    let t9440 = t2880 * t1535;
    let t9441 = t9440 * t2876;
    let t9444 = t1539 * t2869;
    let t9448 = t1539 * t2876;
    let t9449 = t1145 * t9448;
    let t9452 = t1535 * t2876;
    let t9453 = t1145 * t9452;
    let t9458 = t2884 * t1530;
    (t9440, t9441, t9444, t9448, t9449, t9453, t9458)
}
