//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1242/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1242(t40185: f64, t41734: f64, t41735: f64, t41736: f64, t43602: f64, t43606: f64, t43609: f64, t43612: f64, t43616: f64, t43619: f64, t43622: f64, t43625: f64) -> f64 {
    let t44471 = t41734 + 0.46230515946956099003e0_f64 * t43602 + t41735 + t41736 - 0.62295486109113302474e-1_f64 * t40185 + 0.43663693315433241794e-2_f64 * t43606 - 0.93149212406257582492e-1_f64 * t43609 - 0.52396431978519890152e-1_f64 * t43612 + 0.55889527443754549496e0_f64 * t43616 + 0.2600466522016280569e0_f64 * t43619 - 0.34672886960217074252e0_f64 * t43622 - 0.10401866088065122276e1_f64 * t43625;
    t44471
}
