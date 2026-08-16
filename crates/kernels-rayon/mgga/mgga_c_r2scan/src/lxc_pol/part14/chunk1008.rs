//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1008/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1008(t11687: f64, t10730: f64, t10732: f64, t10745: f64, t10770: f64, t11393: f64, t11399: f64, t11672: f64, t11676: f64, t11679: f64, t11681: f64, t11684: f64) -> f64 {
    let t12132 = 0.23115257973478049502e0_f64 * t11687;
    let t12133 = 0.47609969197673950973e-2_f64 * t10730 - 0.47609969197673950973e-2_f64 * t10732 - t11393 + t10745 + t11399 + 0.32927245914677557992e0_f64 * t11672 + 0.47609969197673950973e-2_f64 * t10770 - 0.13099107994629972538e-1_f64 * t11676 + 0.43663693315433241794e-2_f64 * t11679 - 0.47609969197673950973e-2_f64 * t11681 - 0.87327386630866483588e-2_f64 * t11684 + t12132;
    t12133
}
