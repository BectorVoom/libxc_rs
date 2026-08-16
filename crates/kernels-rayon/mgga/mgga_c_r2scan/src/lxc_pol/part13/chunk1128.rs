//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1128/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1128(t39506: f64, t39509: f64, t39512: f64, t39514: f64, t39517: f64, t39520: f64, t39523: f64, t39524: f64, t39526: f64, t39529: f64, t39532: f64, t39535: f64) -> f64 {
    let t39537 = 0.32927245914677557994e0_f64 * t39506 + 0.16463622957338778997e0_f64 * t39509 + t39512 - 0.27439371595564631661e-1_f64 * t39514 + 0.21831846657716620896e-2_f64 * t39517 + 0.26198215989259945076e-1_f64 * t39520 + t39523 + 0.5200933044032561138e0_f64 * t39524 - 0.87327386630866483584e-2_f64 * t39526 - 0.87327386630866483584e-2_f64 * t39529 + 0.13099107994629972538e-1_f64 * t39532 - 0.13099107994629972538e-1_f64 * t39535;
    t39537
}
