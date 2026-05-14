//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 975/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk975<F: Float>(t9274: F, t9271: F, t9292: F, t9353: F, t9357: F, t9360: F, t9368: F, t9370: F, t9375: F, t9377: F, t9380: F, t9382: F, t9478: F, t976: F, t3541: F, t967: F) -> (F, F, F, F) {
    let t9485 = 0.103295e1 * t9274;
    let t9491 = 0.62517e0 * t9353 + 0.312585e0 * t9357 + 0.34731666666666666667e0 * t9360 + 0.68863333333333333333e0 * t9271 + 0.3529725e1 * t9368 + 0.6311625e0 * t9370 - t9485 + 0.1549425e1 * t9292 - 0.3529725e1 * t9375 - 0.17648625e1 * t9377 + 0.6311625e0 * t9380 + 0.31558125e0 * t9382;
    let t9492 = t9478 + t9491;
    let t9493 = t9492 * t976;
    let t9496 = t3541 * t967;
    (t9485, t9492, t9493, t9496)
}
