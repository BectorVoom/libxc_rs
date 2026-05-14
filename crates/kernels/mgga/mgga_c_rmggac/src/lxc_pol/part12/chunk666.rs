//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 666/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk666<F: Float>(t551: F, t876: F, t570: F, t794: F, t1652: F, t352: F, t866: F, t848: F, t1587: F, t838: F, t874: F, t839: F, t558: F, t209: F, t5666: F, t1540: F, t325: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t27102 = t551 * t876;
    let t27111 = t570 * t794;
    let t27120 = t1652 * t352;
    let t27124 = t551 * t866;
    let t27136 = t570 * t848;
    let t27146 = t1587 * t352;
    let t27176 = t838 * t874;
    let t27177 = t570 * t839;
    let t27326 = t558 * t876;
    let t27724 = t5666 * t209;
    let t28295 = t1540 * t325;
    (t27102, t27111, t27120, t27124, t27136, t27146, t27176, t27177, t27326, t27724, t28295)
}
