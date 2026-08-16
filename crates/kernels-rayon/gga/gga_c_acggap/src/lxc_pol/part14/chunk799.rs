//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 799/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk799(t7346: f64, t8952: f64, t1165: f64, t604: f64, t8901: f64, t7337: f64, t7771: f64, t7773: f64, t7776: f64, t7782: f64, t7788: f64, t7789: f64, t7797: f64, t7801: f64, t7803: f64, t7806: f64, t8943: f64, t8945: f64, t8949: f64) -> (f64, f64, f64) {
    let t8953 = t7346 * t8952;
    let t8956 = t1165 * t604 * t8901;
    let t8957 = t7337 * t8956;
    let t8959 = -0.3572834843172478081e-3_f64 * t7771 - 0.64311027177104605458e-3_f64 * t7773 - t7776 + t7782 - t7788 + 0.10718504529517434243e-3_f64 * t7789 + 0.7145669686344956162e-4_f64 * t7797 + t7801 - t7803 - t7806 + t8943 / 96.0_f64 - 7.0_f64 / 288.0_f64 * t8945 + 0.21437009059034868486e-3_f64 * t8949 - 0.15724046144802076034e-3_f64 * t8953 - 0.7862023072401038017e-3_f64 * t8957;
    (t8953, t8956, t8959)
}
