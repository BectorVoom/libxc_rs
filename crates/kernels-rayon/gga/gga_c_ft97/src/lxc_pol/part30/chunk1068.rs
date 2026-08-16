//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1068/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1068(t140653: f64, t3864: f64, t10052: f64, t3972: f64, t7546: f64, t10002: f64, t35737: f64, t24412: f64, t27986: f64, t1403: f64, t141560: f64, t193: f64, t24231: f64, t27992: f64, t28043: f64, t33259: f64, t33499: f64, t35285: f64, t35297: f64, t35751: f64, t35779: f64, t5996: f64, t6002: f64, t6064: f64, t6745: f64, t684: f64, t771: f64) -> (f64, f64, f64, f64, f64) {
    let t151387 = t140653 * t3864;
    let t151405 = t10052 * t7546 * t3972;
    let t151407 = t10002 * t35737;
    let t151409 = t24412 * t27986;
    let t151411 = -t141560 / 18.0_f64 + t6745 * t33259 / 3.0_f64 - t5996 * t35297 / 3.0_f64 + 4.0_f64 * t151387 + 2.0_f64 / 9.0_f64 * t6002 * t24231 * t27992 + t33499 * t28043 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t6002 * t24231 * t35285 * t684 + t35779 * t6064 / 6.0_f64 + t1403 * t193 * t35751 * t771 / 6.0_f64 - 12.0_f64 * t151405 + 8.0_f64 * t151407 + 8.0_f64 * t151409;
    (t151387, t151405, t151407, t151409, t151411)
}
