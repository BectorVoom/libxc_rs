//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1162/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1162(t3602: f64, t37755: f64, t6064: f64, t3606: f64, t6087: f64, t37983: f64, t39900: f64, t39903: f64, t39906: f64, t39908: f64, t39912: f64, t39914: f64, t39916: f64, t39920: f64, t39924: f64) -> f64 {
    let t39927 = t37755 * t3602 * t6064;
    let t39930 = t37755 * t3606 * t6087;
    let t39932 = -t39900 - 0.13972381860938637373e0_f64 * t39903 + 0.67533178994536747305e0_f64 * t39906 - 0.32927245914677557993e-1_f64 * t39908 - t39912 - 0.5200933044032561138e0_f64 * t39914 - 0.54878743191129263322e-1_f64 * t39916 + 0.19514881078765566037e-1_f64 * t37983 + 0.14282990759302185291e-1_f64 * t39920 + 0.87327386630866483584e-2_f64 * t39924 + 0.13099107994629972538e-1_f64 * t39927 + 0.13099107994629972538e-1_f64 * t39930;
    t39932
}
