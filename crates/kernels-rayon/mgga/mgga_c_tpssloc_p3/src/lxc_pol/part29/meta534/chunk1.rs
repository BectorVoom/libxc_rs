//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1918/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1918(t26323: f64, t6936: f64, t22856: f64, t22859: f64, t22860: f64, t22864: f64, t22868: f64, t26306: f64, t26310: f64, t26312: f64, t26314: f64, t26320: f64) -> f64 {
    let t26324 = t6936 * t26323;
    let t26326 = t26306 / 384.0_f64 + t26310 / 768.0_f64 - t26312 / 1536.0_f64 + t26314 / 384.0_f64 + 0.33643963411783659045e-4_f64 * t22856 + t22859 - 7.0_f64 / 2304.0_f64 * t22860 + t22864 + t22868 + 0.40372756094140390854e-3_f64 * t26320 - 0.20186378047070195427e-3_f64 * t26324;
    t26326
}
