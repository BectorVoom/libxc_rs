//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 768/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk768(t1043: f64, t4104: f64, t1024: f64, t1518: f64, t2913: f64, t1042: f64, t2911: f64, t2836: f64, t2917: f64, t4044: f64, t4049: f64, t4054: f64, t4058: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4105 = t4104 * t1043;
    let t4107 = 1.0_f64 * t1024 * t4105;
    let t4108 = t1518 * t2913;
    let t4109 = t4108 * t1042;
    let t4111 = 0.16081979498692535067e2_f64 * t2911 * t4109;
    let t4117 = t2917 - 0.57077777777777777777e-2_f64 * t2836 - 0.57077777777777777777e-2_f64 * t4044 - 0.11415555555555555555e-1_f64 * t4049 + 0.34246666666666666666e-1_f64 * t4054 + 0.17123333333333333333e-1_f64 * t4058;
    (t4105, t4107, t4108, t4109, t4111, t4117)
}
