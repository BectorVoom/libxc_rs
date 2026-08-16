//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 15/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk15(t22: f64, t23: f64, t6: f64, t15: f64, t17: f64, t19: f64, t14: f64, rho0: f64, rho1: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26 = t22 * t6 / t23;
    let t27 = 0.123235e0_f64 * t26;
    let t28 = 0.379785e1_f64 * t15 + t17 + t19 + t27;
    let t31 = 1.0_f64 + 0.16081979498692535067e2_f64 / t28;
    let t32 = f64::ln(t31);
    let t34 = 0.621814e-1_f64 * t14 * t32;
    let t35 = rho0 - rho1;
    (t26, t27, t28, t31, t32, t34, t35)
}
