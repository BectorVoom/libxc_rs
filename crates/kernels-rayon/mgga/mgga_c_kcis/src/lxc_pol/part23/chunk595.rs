//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 595/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk595(t1378: f64, t5732: f64, t286: f64, t1368: f64, t1373: f64, t1382: f64, t1930: f64, t3969: f64, t3972: f64, t3975: f64, t493: f64, t5689: f64, t5691: f64, t5699: f64, t5702: f64, t5706: f64, t5710: f64, t5715: f64, t5719: f64, t5723: f64, t5728: f64) -> (f64, f64) {
    let t5733 = t1378 * t5732;
    let t5734 = t286 * t5733;
    let t5737 = -t5689 / 108.0_f64 - t5691 * t1373 / 108.0_f64 + t1930 * t1382 / 36.0_f64 - t3969 + t3972 / 864.0_f64 - t3975 / 288.0_f64 + t5699 / 864.0_f64 + t1368 * t5702 / 216.0_f64 - t1368 * t5706 / 288.0_f64 - t1368 * t5710 / 144.0_f64 - t1368 * t5715 / 144.0_f64 - t5719 / 288.0_f64 - t1368 * t5723 / 288.0_f64 + t1368 * t5728 / 48.0_f64 - t493 * t5734 / 96.0_f64;
    (t5733, t5737)
}
