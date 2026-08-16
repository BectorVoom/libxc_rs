//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1219/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1219(t113: f64, t1259: f64, t1274: f64, t20489: f64, t21801: f64, t21802: f64, t21806: f64, t21812: f64, t21815: f64, t21899: f64, t22480: f64, t332: f64, t333: f64, t4322: f64, t4635: f64, t5: f64, t5430: f64, t86571: f64, t889: f64, t91145: f64, t91216: f64, t91269: f64, t91334: f64, t91387: f64, t91423: f64, t992: f64) -> f64 {
    let t91432 = t5 * (t91145 + t91216) * t332 * t113 / 4.0_f64 + t5 * t22480 * t992 + t5 * t333 * t86571 / 4.0_f64 + 3.0_f64 / 2.0_f64 * t5 * t5430 * t4635 + t889 * t21801 * t992 + 3.0_f64 * t4322 * t21812 + t4322 * t21802 + t5 * t1259 * t20489 + 3.0_f64 * t4322 * t21815 + t889 * t1274 * t20489 + 3.0_f64 * t4322 * t21806 + t889 * (t91269 + t91334 + t91387 + t91423) * t332 * t113 / 4.0_f64 + t889 * t21899 * t992;
    t91432
}
