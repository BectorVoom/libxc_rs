//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1183/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1183<F: Float>(t3018: F, t3020: F, t5566: F, t7857: F, t47: F, t8: F, t420: F, t22568: F, t25709: F, t25714: F, t92819: F, t6431: F, t7983: F, t22632: F, t22761: F, t25787: F) -> (F, F, F, F, F, F) {
    let t101247 = t7857 * t3018 * t3020 * t5566;
    let t101248 = t8 * t47;
    let t101249 = t101248 * t420;
    let t101279 = t22568 * t25709;
    let t101282 = t92819 * t25714;
    let t101285 = t7983 * t6431;
    let t101295 = t22761 * t22632 * t25787;
    (t101247, t101249, t101279, t101282, t101285, t101295)
}
