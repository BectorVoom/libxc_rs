//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 754/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk754(t21066: f64, t21085: f64, t184: f64, t21: f64, t1078: f64, t4888: f64, t3664: f64, t1064: f64, t1080: f64, t16612: f64, t185: f64, t20044: f64, t20990: f64, t20996: f64, t21002: f64, t21005: f64, t21008: f64, t3601: f64, t4431: f64, t4845: f64, t4890: f64, t4895: f64, t4898: f64, t5: f64, t623: f64, t920: f64) -> (f64, f64, f64, f64, f64) {
    let t21086 = t21066 + t21085;
    let t21087 = t21086 * t184;
    let t21088 = t21087 * t21;
    let t21091 = t4888 * t1078;
    let t21092 = t21091 * t3664;
    let t21099 = 3.0_f64 / 4.0_f64 * t5 * t4845 * t920 + 3.0_f64 / 4.0_f64 * t5 * t1064 * t4431 + 3.0_f64 / 2.0_f64 * t3601 * t4898 + 3.0_f64 / 4.0_f64 * t3601 * t4890 + t5 * t20990 * t21 / 4.0_f64 + t623 * t20996 / 4.0_f64 + t5 * t185 * t20044 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t623 * t21002 + 3.0_f64 / 4.0_f64 * t623 * t21005 + 3.0_f64 / 4.0_f64 * t623 * t21008 + t623 * t21088 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t623 * t21092 + 3.0_f64 / 4.0_f64 * t3601 * t4895 + 3.0_f64 / 4.0_f64 * t16612 * t1080;
    (t21086, t21087, t21088, t21092, t21099)
}
