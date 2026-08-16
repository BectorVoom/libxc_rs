//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 873/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk873(t11: f64, t5: f64, t5193: f64, t5195: f64, t5198: f64, t7637: f64, t7641: f64, t7854: f64, t7858: f64, t7862: f64, t7869: f64, t7880: f64, t7884: f64, t7895: f64, t7905: f64, t7910: f64, param_eta: f64) -> f64 {
    let t7916 = -t5193 + 40.0_f64 / 9.0_f64 * t5195 - 5.0_f64 / 3.0_f64 * t5198 + 20.0_f64 / 9.0_f64 * t7637 - t7641 + 5.0_f64 * t5 * t11 * t7854 - 45.0_f64 * param_eta * (t7858 + t7862 + t7869 + t7880 + t7884 + t7895 + t7905 + t7910);
    t7916
}
