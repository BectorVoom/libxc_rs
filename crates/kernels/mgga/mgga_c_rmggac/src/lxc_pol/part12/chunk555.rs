//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 555/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk555<F: Float>(t7442: F, t645: F, t848: F, t903: F, t209: F, t352: F, t476: F) -> (F, F, F, F) {
    let t7443 = F::cast_from(0.2993560425465952141e-1_f64) * t7442;
    let t7444 = t645 * t848;
    let t7445 = t903 * t7444;
    let t7446 = F::cast_from(0.44903406381989282115e-1_f64) * t7445;
    let t7448 = t352 * t476 * t209;
    (t7443, t7444, t7446, t7448)
}
