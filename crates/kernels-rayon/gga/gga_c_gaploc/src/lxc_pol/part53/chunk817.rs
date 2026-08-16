//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 817/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk817(t2365: f64, t28652: f64, t6111: f64, t40820: f64, t900: f64, t28973: f64, t28028: f64, t959: f64, t10024: f64, t10037: f64, t22624: f64, t7427: f64, t9438: f64) -> (f64, f64, f64, f64, f64) {
    let t41337 = t6111 * t2365 * t28652;
    let t41339 = t900 * t40820;
    let t41340 = t28973 * t41339;
    let t41342 = t28028 * t959;
    let t41405 = t10037 * t10024;
    let t41408 = t7427 * t9438 * t22624;
    (t41337, t41340, t41342, t41405, t41408)
}
