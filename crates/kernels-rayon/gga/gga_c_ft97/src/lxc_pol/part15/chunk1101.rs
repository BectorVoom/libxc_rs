//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1101/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1101(t1079: f64, t1080: f64, t16612: f64, t184: f64, t20044: f64, t20995: f64, t20996: f64, t21: f64, t21002: f64, t21005: f64, t21008: f64, t21087: f64, t21088: f64, t3601: f64, t4888: f64, t4890: f64, t4895: f64, t623: f64, t78929: f64, t87868: f64, t87906: f64, t87941: f64, t88021: f64, t920: f64) -> f64 {
    let t88051 = t623 * (t87868 + t87906 + t87941 + t88021) * t184 * t21 / 4.0_f64 + t623 * t1079 * t20044 + 3.0_f64 * t3601 * t21008 + t78929 * t1080 + t3601 * t20996 + 3.0_f64 * t3601 * t21005 + t623 * t21087 * t920 + 3.0_f64 * t623 * t1079 * t920 * t4888 + 3.0_f64 / 2.0_f64 * t16612 * t4895 + 3.0_f64 / 2.0_f64 * t16612 * t4890 + t623 * t20995 * t920 + 3.0_f64 * t3601 * t21002 + t3601 * t21088;
    t88051
}
