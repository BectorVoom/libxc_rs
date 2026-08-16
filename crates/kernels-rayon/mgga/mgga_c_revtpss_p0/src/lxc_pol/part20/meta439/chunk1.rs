//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1664/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1664(t44982: f64, t45016: f64, t45053: f64, t45327: f64, t13062: f64, t13064: f64, t3172: f64, t1012: f64, t1042: f64, t1222: f64, t1225: f64, t1247: f64, t1250: f64, t12922: f64, t12956: f64, t13079: f64, t247: f64, t3368: f64, t3372: f64, t3611: f64, t3719: f64, t3720: f64, t39443: f64, t39457: f64, t44552: f64, t44944: f64, t44949: f64, t44952: f64, t44959: f64, t44965: f64, t44972: f64, t44974: f64, t44980: f64, t482: f64, t5384: f64) -> (f64, f64) {
    let t45329 = t44982 + t45016 + t45053 + t45327;
    let t45346 = t13062 * t3172 * t13064;
    let t45348 = 0.17149607247227894789e-2_f64 * t5384 * t247 * t3719 * t44944 + 0.22866142996303859718e-2_f64 * t44949 - 0.25724410870841842184e-2_f64 * t44952 * t3720 * t44552 * t3611 + 35.0_f64 / 972.0_f64 * t1222 * t1012 * t44959 * t39443 - t44965 / 36.0_f64 - t1222 * t1012 * t1225 * t39457 / 288.0_f64 - 7.0_f64 / 486.0_f64 * t44972 - 7.0_f64 / 54.0_f64 * t1222 * t1012 * t44974 * t39443 - t44980 / 162.0_f64 + 0.21437009059034868486e-3_f64 * t1247 * t1042 * t482 * t45329 * t1250 - 0.17149607247227894789e-2_f64 * t5384 * t1042 * t13079 * t3372 - 0.34299214494455789578e-2_f64 * t5384 * t1042 * t13079 * t3368 + 0.34299214494455789578e-2_f64 * t12956 * t12922 + 0.57165357490759649296e-3_f64 * t45346;
    (t45329, t45348)
}
