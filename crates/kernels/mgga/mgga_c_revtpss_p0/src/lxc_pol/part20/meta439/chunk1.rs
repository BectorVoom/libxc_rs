//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1664/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1664<F: Float>(t44982: F, t45016: F, t45053: F, t45327: F, t13062: F, t13064: F, t3172: F, t1012: F, t1042: F, t1222: F, t1225: F, t1247: F, t1250: F, t12922: F, t12956: F, t13079: F, t247: F, t3368: F, t3372: F, t3611: F, t3719: F, t3720: F, t39443: F, t39457: F, t44552: F, t44944: F, t44949: F, t44952: F, t44959: F, t44965: F, t44972: F, t44974: F, t44980: F, t482: F, t5384: F) -> (F, F) {
    let t45329 = t44982 + t45016 + t45053 + t45327;
    let t45346 = t13062 * t3172 * t13064;
    let t45348 = F::cast_from(0.17149607247227894789e-2_f64) * t5384 * t247 * t3719 * t44944 + F::cast_from(0.22866142996303859718e-2_f64) * t44949 - F::cast_from(0.25724410870841842184e-2_f64) * t44952 * t3720 * t44552 * t3611 + F::new(35.0) / F::new(972.0) * t1222 * t1012 * t44959 * t39443 - t44965 / F::new(36.0) - t1222 * t1012 * t1225 * t39457 / F::new(288.0) - F::new(7.0) / F::new(486.0) * t44972 - F::new(7.0) / F::new(54.0) * t1222 * t1012 * t44974 * t39443 - t44980 / F::new(162.0) + F::cast_from(0.21437009059034868486e-3_f64) * t1247 * t1042 * t482 * t45329 * t1250 - F::cast_from(0.17149607247227894789e-2_f64) * t5384 * t1042 * t13079 * t3372 - F::cast_from(0.34299214494455789578e-2_f64) * t5384 * t1042 * t13079 * t3368 + F::cast_from(0.34299214494455789578e-2_f64) * t12956 * t12922 + F::cast_from(0.57165357490759649296e-3_f64) * t45346;
    (t45329, t45348)
}
