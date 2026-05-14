//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 757/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk757<F: Float>(t40920: F, t5162: F, t38568: F, t4669: F, t1587: F, t2064: F, t793: F, t118: F, t2001: F, t352: F, t38523: F, t34884: F, t9118: F, t2283: F, t34881: F, t2286: F, t7939: F) -> (F, F, F, F, F, F, F, F) {
    let t41534 = t5162 * t40920;
    let t41536 = t4669 * t38568;
    let t41548 = t2064 * t1587;
    let t41549 = t793 * t41548;
    let t41576 = t2001 * t118 * t38523 * t352;
    let t41579 = t34884 * t9118;
    let t41581 = t34881 * t2283;
    let t41585 = t7939 * t2286;
    (t41534, t41536, t41548, t41549, t41576, t41579, t41581, t41585)
}
