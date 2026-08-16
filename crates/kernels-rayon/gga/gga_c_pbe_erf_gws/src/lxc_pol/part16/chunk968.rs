//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 968/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk968(t43: f64, t50: f64, t8079: f64, t8082: f64, t8084: f64, t8086: f64, t8088: f64, t8091: f64, t8094: f64, t8096: f64, t8098: f64, t8100: f64, zeta_threshold: f64) -> f64 {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t8565 = piecewise3(t44, 0.0_f64, 8.0_f64 / 27.0_f64 * t8079 - 8.0_f64 / 9.0_f64 * t8082 - 2.0_f64 / 9.0_f64 * t8084 + 4.0_f64 / 3.0_f64 * t8086 - 4.0_f64 * t8088);
    let t8572 = piecewise3(t51, 0.0_f64, 8.0_f64 / 27.0_f64 * t8091 + 8.0_f64 / 9.0_f64 * t8094 - 2.0_f64 / 9.0_f64 * t8096 - 4.0_f64 / 3.0_f64 * t8098 + 4.0_f64 * t8100);
    let t8574 = t8565 / 2.0_f64 + t8572 / 2.0_f64;
    t8574
}
