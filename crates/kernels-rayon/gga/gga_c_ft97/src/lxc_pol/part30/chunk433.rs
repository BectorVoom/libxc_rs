//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 433/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk433(t317: f64, t7022: f64, t193: f64, t1253: f64, t1477: f64, t1091: f64, t6273: f64, t2874: f64, t1212: f64, t1476: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7023 = t7022 * t317;
    let t7024 = t193 * t7023;
    let t7027 = t1477 * t1253;
    let t7028 = t193 * t7027;
    let t7032 = t6273 * t1091;
    let t7033 = t2874 * t7032;
    let t7036 = t1476 * t1212;
    (t7023, t7024, t7027, t7028, t7032, t7033, t7036)
}
