//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 984/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk984(t23477: f64, t42945: f64, t4820: f64, t10667: f64, t123: f64, t883: f64, t2684: f64, t2685: f64, t10628: f64, t549: f64, t6111: f64, t24505: f64, t9438: f64) -> (f64, f64, f64, f64, f64) {
    let t43708 = 0.23833659967900284446e0_f64 * t23477 * t4820 * t42945;
    let t43710 = t10667 * t123 * t883;
    let t43712 = t2684 * t2685 * t43710;
    let t43715 = t6111 * t549 * t10628;
    let t43716 = 0.11916829983950142223e0_f64 * t43715;
    let t43718 = t2684 * t9438 * t24505;
    (t43708, t43710, t43712, t43716, t43718)
}
