//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1295/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1295(t14015: f64, t9540: f64, t9619: f64, t14063: f64, t8962: f64, t854: f64, t51201: f64, t14064: f64, t3113: f64, t3120: f64, t14031: f64, t9372: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54019 = t14015 * t9540;
    let t54021 = t14015 * t9619;
    let t54023 = t14063 * t8962;
    let t54024 = t854 * t54023;
    let t54026 = 119.0_f64 / 1728.0_f64 * t51201;
    let t54027 = t3113 * t14064;
    let t54029 = t3120 * t14064;
    let t54031 = t14031 * t9372;
    (t54019, t54021, t54024, t54026, t54027, t54029, t54031)
}
