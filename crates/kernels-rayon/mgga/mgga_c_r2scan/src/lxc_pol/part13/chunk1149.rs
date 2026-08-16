//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1149/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1149(t37859: f64, t37881: f64, t39759: f64, t39763: f64, t39765: f64, t39768: f64, t39771: f64, t39772: f64, t39775: f64, t39778: f64, t39780: f64, t39782: f64) -> f64 {
    let t39784 = 0.23115257973478049502e0_f64 * t37859 + 0.47609969197673950972e-2_f64 * t37881 - 0.5200933044032561138e0_f64 * t39759 - t39763 - 0.2600466522016280569e0_f64 * t39765 - 0.2600466522016280569e0_f64 * t39768 + t39771 - 0.42683466926433871473e0_f64 * t39772 - 0.87327386630866483584e-2_f64 * t39775 - 0.13099107994629972538e-1_f64 * t39778 - 0.13099107994629972538e-1_f64 * t39780 - 0.5239643197851989015e-1_f64 * t39782;
    t39784
}
