//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1300/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1300(t10257: f64, t10259: f64, t12371: f64, t16379: f64, t21910: f64, t21911: f64, t21912: f64, t21913: f64, t21914: f64, t21917: f64, t21920: f64, t21921: f64, t21922: f64, t21923: f64, t8016: f64, t8018: f64, t8023: f64) -> f64 {
    let t50812 = 0.79007158810260824916e-1_f64 * t12371 - 48.0_f64 * t10257 - 48.0_f64 * t10259 - 0.70178680769462448852e1_f64 * t8016 - 0.2077890707925103596e3_f64 * t8018 + 0.14035736153892489771e2_f64 * t8023 + t16379 - t21910 - t21911 - t21912 + t21913 + t21914 + t21917 - t21920 - t21921 - t21922 - t21923;
    t50812
}
