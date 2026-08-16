//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 439/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk439<F: Float>(t1186: F, t4569: F, t1243: F, t195: F, t194: F, t498: F, t1144: F, t500: F, t325: F, t892: F) -> (F, F, F, F, F) {
    let t4570 = t4569 * t1186;
    let t4580 = t195 * t1243;
    let t4585 = t194 * t498;
    let t4586 = t500 * t1144;
    let t4601 = t892 * t325;
    (t4570, t4580, t4585, t4586, t4601)
}
