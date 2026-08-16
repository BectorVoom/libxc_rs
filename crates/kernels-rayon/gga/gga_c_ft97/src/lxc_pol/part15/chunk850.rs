//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 850/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk850(t22479: f64, t332: f64, t113: f64, t1259: f64, t1275: f64, t19920: f64, t20489: f64, t21802: f64, t21806: f64, t21812: f64, t21815: f64, t21818: f64, t21900: f64, t333: f64, t4322: f64, t4635: f64, t5: f64, t5430: f64, t5475: f64, t5480: f64, t5483: f64, t889: f64, t992: f64) -> (f64, f64) {
    let t22480 = t22479 * t332;
    let t22487 = 3.0_f64 / 4.0_f64 * t19920 * t1275 + 3.0_f64 / 4.0_f64 * t5 * t1259 * t4635 + t889 * t21802 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t889 * t21806 + t5 * t333 * t20489 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t889 * t21812 + 3.0_f64 / 4.0_f64 * t889 * t21815 + 3.0_f64 / 4.0_f64 * t889 * t21818 + t889 * t21900 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t4322 * t5480 + 3.0_f64 / 4.0_f64 * t4322 * t5475 + 3.0_f64 / 2.0_f64 * t4322 * t5483 + t5 * t22480 * t113 / 4.0_f64 + 3.0_f64 / 4.0_f64 * t5 * t5430 * t992;
    (t22480, t22487)
}
