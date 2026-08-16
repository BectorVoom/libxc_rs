//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 808/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk808(t8876: f64, t8879: f64, t8882: f64, t8898: f64, t7686: f64, t7699: f64, t7710: f64, t7714: f64, t7722: f64, t8235: f64, t8240: f64, t8247: f64, t8249: f64, t8885: f64, t8890: f64) -> f64 {
    let t9328 = t8876 / 32.0_f64;
    let t9329 = t8879 / 96.0_f64;
    let t9331 = 0.5603125e-1_f64 * t8882;
    let t9335 = 0.21437009059034868486e-3_f64 * t8898;
    let t9336 = -t9328 - t9329 + t8235 + 0.40015750243531754507e-2_f64 * t7686 - t8240 - t9331 + t8885 / 24.0_f64 + t8890 / 24.0_f64 - t7699 + 0.62896184579208304137e-3_f64 * t7710 - t7714 - t8247 - t7722 - t8249 + t9335;
    t9336
}
