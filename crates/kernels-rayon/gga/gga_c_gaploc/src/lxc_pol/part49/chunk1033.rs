//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1033/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1033(t1645: f64, t3025: f64, t9689: f64, t13020: f64, t5771: f64, t23477: f64, t42945: f64, t4820: f64, t10667: f64, t123: f64, t883: f64, t2684: f64, t2685: f64) -> (f64, f64, f64, f64, f64) {
    let t43698 = 0.10725146985555128001e1_f64 * t3025 * t1645 * t9689;
    let t43699 = t5771 * t13020;
    let t43708 = 0.23833659967900284446e0_f64 * t23477 * t4820 * t42945;
    let t43710 = t10667 * t123 * t883;
    let t43712 = t2684 * t2685 * t43710;
    (t43698, t43699, t43708, t43710, t43712)
}
