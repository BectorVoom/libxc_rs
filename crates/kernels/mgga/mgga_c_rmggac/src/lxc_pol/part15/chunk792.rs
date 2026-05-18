//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 792/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk792<F: Float>(t2004: F, t38638: F, t2007: F, t1987: F, t1990: F, t1652: F, t7778: F, t739: F, t7364: F, t8576: F, t16156: F, t8508: F) -> (F, F, F, F, F, F, F, F) {
    let t38639 = t38638 * t2004;
    let t38640 = F::new(0.19863479950205658386e-4) * t38639;
    let t38643 = t38638 * t2007;
    let t38645 = t38638 * t1987;
    let t38647 = t38638 * t1990;
    let t38648 = F::new(0.19863479950205658386e-4) * t38647;
    let t38674 = t7778 * t1652;
    let t38675 = t739 * t38674;
    let t38676 = F::new(0.79828278012425390426e-1) * t38675;
    let t38701 = t8576 * t7364;
    let t38704 = t16156 * t8508;
    (t38640, t38643, t38645, t38648, t38674, t38676, t38701, t38704)
}
