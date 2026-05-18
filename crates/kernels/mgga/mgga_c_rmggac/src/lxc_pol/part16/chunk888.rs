//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 888/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk888<F: Float>(t1734: F, t352: F, t3928: F, t6418: F, t645: F, t1550: F, t2060: F, t558: F, t8708: F, t262: F, t7198: F, t570: F) -> (F, F, F, F, F, F, F) {
    let t44713 = t1734 * t352;
    let t44724 = t3928 * t645 * t6418;
    let t44727 = t1550 * t2060 * t44713;
    let t44732 = t8708 * t558;
    let t44733 = t262 * t44732;
    let t44734 = t7198 * t44733;
    let t44736 = t8708 * t570;
    (t44713, t44724, t44727, t44732, t44733, t44734, t44736)
}
