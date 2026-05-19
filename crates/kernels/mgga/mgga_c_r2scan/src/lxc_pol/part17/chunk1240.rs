//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1240/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1240<F: Float>(t38622: F, t40107: F, t40109: F, t43559: F, t43561: F, t43565: F, t43569: F, t43572: F, t43575: F, t43577: F, t43579: F, t43581: F) -> F {
    let t44452 = -F::cast_from(0.2600466522016280569e0_f64) * t43559 - F::cast_from(0.5200933044032561138e0_f64) * t43561 - t38622 - F::cast_from(0.2600466522016280569e0_f64) * t43565 + F::cast_from(0.11708928647259339622e0_f64) * t40107 - F::cast_from(0.54878743191129263322e-1_f64) * t43569 - F::cast_from(0.52009330440325611378e0_f64) * t43572 - F::cast_from(0.52009330440325611378e0_f64) * t43575 + F::cast_from(0.5200933044032561138e0_f64) * t43577 + F::cast_from(0.20803732176130244552e1_f64) * t43579 + F::cast_from(0.16951189180550569635e1_f64) * t40109 - F::cast_from(0.97574405393827830187e-2_f64) * t43581;
    t44452
}
