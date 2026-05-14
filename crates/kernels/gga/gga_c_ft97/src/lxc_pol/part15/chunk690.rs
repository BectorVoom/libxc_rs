//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 690/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk690<F: Float>(t21066: F, t21085: F, t184: F, t21: F, t1078: F, t4888: F, t3664: F, t1064: F, t1080: F, t16612: F, t185: F, t20044: F, t20990: F, t20996: F, t21002: F, t21005: F, t21008: F, t3601: F, t4431: F, t4845: F, t4890: F, t4895: F, t4898: F, t5: F, t623: F, t920: F) -> (F, F, F, F, F) {
    let t21086 = t21066 + t21085;
    let t21087 = t21086 * t184;
    let t21088 = t21087 * t21;
    let t21091 = t4888 * t1078;
    let t21092 = t21091 * t3664;
    let t21099 = 3.0 / 4.0 * t5 * t4845 * t920 + 3.0 / 4.0 * t5 * t1064 * t4431 + 3.0 / 2.0 * t3601 * t4898 + 3.0 / 4.0 * t3601 * t4890 + t5 * t20990 * t21 / 4.0 + t623 * t20996 / 4.0 + t5 * t185 * t20044 / 4.0 + 3.0 / 4.0 * t623 * t21002 + 3.0 / 4.0 * t623 * t21005 + 3.0 / 4.0 * t623 * t21008 + t623 * t21088 / 4.0 + 3.0 / 4.0 * t623 * t21092 + 3.0 / 4.0 * t3601 * t4895 + 3.0 / 4.0 * t16612 * t1080;
    (t21086, t21087, t21088, t21092, t21099)
}
