//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 700/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk700<F: Float>(t35586: F, t797: F, t2123: F, t4616: F, t265: F, t874: F, t20: F, t2018: F, t2021: F, t4729: F, t2131: F, t4036: F, t1969: F, t8516: F, t7229: F, t7243: F) -> (F, F, F, F, F, F, F) {
    let t36286 = t797 * t35586;
    let t36288 = t4616 * t2123;
    let t36292 = t874 * t265;
    let t36330 = t4729 * t20 * t2018 * t2021;
    let t36331 = 0.91462949374725084942e-3 * t36330;
    let t36332 = t4036 * t2131;
    let t36336 = t8516 * t1969;
    let t36343 = t7229 * t7243;
    (t36286, t36288, t36292, t36331, t36332, t36336, t36343)
}
