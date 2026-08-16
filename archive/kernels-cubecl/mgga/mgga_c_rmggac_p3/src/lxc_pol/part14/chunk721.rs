//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 721/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk721<F: Float>(t570: F, t839: F, t558: F, t876: F, t209: F, t5666: F, t1540: F, t325: F, t107: F, t1539: F, t837: F, t874: F) -> (F, F, F, F, F, F) {
    let t27177 = t570 * t839;
    let t27326 = t558 * t876;
    let t27724 = t5666 * t209;
    let t28295 = t1540 * t325;
    let t28317 = t1539 * t107;
    let t29837 = t837 * t874;
    (t27177, t27326, t27724, t28295, t28317, t29837)
}
