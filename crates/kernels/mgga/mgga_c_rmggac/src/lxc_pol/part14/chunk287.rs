//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 287/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk287<F: Float>(t1046: F, t195: F, t618: F, t1412: F, t181: F, t446: F, t589: F, t1011: F, t1014: F, t1027: F, t1029: F, t1374: F, t1414: F, t1416: F, t1418: F, t1420: F, t1421: F, t948: F, t975: F, t982: F) -> (F, F, F, F, F) {
    let t1424 = F::cast_from(0.5848223622634646207e0_f64) * t1046;
    let t1425 = t195 * t618;
    let t1429 = F::cast_from(0.19751673498613801407e-1_f64) * t1412 * t181;
    let t1430 = t589 * t446;
    let t1433 = t948 - t975 - t1374 + t1414 + t1416 + t1418 - t1420 + t982 + t1011 - t1014 - t1421 - t1027 + t1029;
    (t1424, t1425, t1429, t1430, t1433)
}
