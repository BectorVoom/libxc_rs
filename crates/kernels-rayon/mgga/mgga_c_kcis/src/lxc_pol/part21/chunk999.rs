//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 999/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk999(t1142: f64, t15092: f64, t1872: f64, t3699: f64, t1291: f64, t5394: f64, t3670: f64, t11223: f64, t11230: f64, t14667: f64, t14670: f64, t14671: f64, t14682: f64, t14685: f64, t3664: f64, t3669: f64, t5360: f64, t5363: f64) -> (f64, f64, f64, f64, f64) {
    let t15093 = t1142 * t15092;
    let t15095 = t1872 * t3699;
    let t15098 = t5394 * t1291;
    let t15101 = t1872 * t3670;
    let t15108 = 4.0_f64 * t11223 * t5363 - 6.0_f64 * t11230 * t15101 + 2.0_f64 * t15095 * t3669 + 4.0_f64 * t15098 * t3669 - 2.0_f64 * t3664 * t5394 - t3699 * t5360 + t14667 - t14670 + t14671 - t14682 - t14685;
    (t15093, t15095, t15098, t15101, t15108)
}
