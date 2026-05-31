//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2236/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2236<F: Float>(t104852: F, t3767: F, t3782: F, t1224: F, t139: F, t29047: F, t5052: F, t3698: F, t5047: F, t16720: F, t16725: F, t17355: F, t17420: F, t17658: F, t17669: F, t17724: F, t26867: F, t26870: F, t29054: F, t29097: F, t5407: F, t97204: F, t97232: F) -> F {
    let t104853 = t3767 * t104852;
    let t104856 = t3782 * t104852;
    let t104863 = t29047 * t139 * t1224 * t5052 / F::cast_from(216.0_f64);
    let t104872 = t29047 * t139 * t3698 * t5047 / F::cast_from(324.0_f64);
    let t104876 = -F::cast_from(0.85748036236139473944e-3_f64) * t26870 * t17724 - F::cast_from(0.57165357490759649296e-3_f64) * t97232 * t5407 - F::cast_from(0.57165357490759649296e-3_f64) * t26867 * t17669 - F::cast_from(0.11433071498151929859e-2_f64) * t104853 * t17658 + F::cast_from(0.57165357490759649296e-3_f64) * t104856 * t17355 + t97204 / F::cast_from(648.0_f64) - t104863 + F::cast_from(0.17149607247227894789e-2_f64) * t29097 * t17420 + t29047 * t29054 * t16720 / F::cast_from(36.0_f64) + t104872 + t29047 * t29054 * t16725 / F::cast_from(108.0_f64);
    t104876
}
