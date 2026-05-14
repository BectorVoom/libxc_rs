//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 621/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk621<F: Float>(t9518: F, t9546: F, t9574: F, t9593: F, t82: F, t72: F, t1685: F, t702: F, t2211: F, t5144: F, t739: F, t5267: F, t884: F, t5888: F, t8041: F, t1356: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9595 = t9518 + t9546 + t9574 + t9593;
    let t9596 = t82 * t9595;
    let t9597 = t72 * t9596;
    let t9598 = t1685 * t702;
    let t9599 = t72 * t9598;
    let t9620 = t2211 * t5144;
    let t9621 = t739 * t9620;
    let t9624 = t2211 * t5267;
    let t9625 = t884 * t9624;
    let t9627 = t8041 * t5888;
    let t9628 = t1356 * t9627;
    (t9595, t9596, t9597, t9598, t9599, t9620, t9621, t9624, t9625, t9627, t9628)
}
