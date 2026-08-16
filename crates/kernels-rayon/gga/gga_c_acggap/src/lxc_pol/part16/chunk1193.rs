//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1193/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1193(t2297: f64, t8791: f64, t13364: f64, t33952: f64, t2046: f64, t336: f64, t5506: f64, t579: f64, t31751: f64, t31752: f64, t36126: f64, t36127: f64, t36132: f64, t36134: f64, t36137: f64, t36152: f64, t36157: f64, t37874: f64, t37877: f64, t37879: f64, t40425: f64, t40427: f64, t40431: f64, t40436: f64) -> (f64, f64) {
    let t40440 = t2297 * t8791;
    let t40442 = t33952 * t13364 * t40440;
    let t40446 = t2046 * t336 * t579 * t5506;
    let t40448 = -t40425 / 192.0_f64 + t36126 + 0.28303283060643736861e-1_f64 * t40427 + 0.15724046144802076034e-2_f64 * t40431 + 0.75475421495049964964e-2_f64 * t36127 - t37874 - t36132 - t36134 - t37877 - 0.62896184579208304136e-3_f64 * t40436 + 0.39624596284901231605e-1_f64 * t36137 - t37879 - t31751 - 0.13208198761633743869e-1_f64 * t31752 - 0.21437009059034868486e-2_f64 * t40442 + t36152 - t40446 / 128.0_f64 - t36157;
    (t40440, t40448)
}
