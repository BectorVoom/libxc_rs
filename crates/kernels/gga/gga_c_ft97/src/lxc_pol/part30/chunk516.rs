//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 516/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk516<F: Float>(t24330: F, t6043: F, t6046: F, t51: F, t1410: F, t695: F, t3758: F, t6056: F, t6055: F, t444: F, t6041: F, t3789: F, t22532: F, t6032: F, t3771: F, t696: F, t70: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24332 = t6043 * t24330 * t6046;
    let t24340 = t51 * sigma2;
    let t24345 = t695 * t1410;
    let t24346 = t3758 * t24345;
    let t24357 = t24330 * t6056;
    let t24358 = t6055 * t24357;
    let t24360 = t6041 * t444;
    let t24361 = t3789 * t24360;
    let t24371 = t6032 * t22532;
    let t24372 = t3771 * t24371;
    let t24378 = t696 * t70;
    (t24332, t24340, t24345, t24346, t24357, t24358, t24361, t24372, t24378)
}
