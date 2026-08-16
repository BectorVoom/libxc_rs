//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1067/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1067(t10844: f64, t10899: f64, t2201: f64, t10848: f64, t2207: f64, t261: f64, t3299: f64, t6507: f64, t3304: f64, t6503: f64, t1582: f64, t2096: f64, t571: f64) -> (f64, f64, f64, f64, f64) {
    let t37838 = t2201 * t10899 * t10844;
    let t37841 = t2207 * t10899 * t10848;
    let t37848 = t3299 * t261 * t6507;
    let t37851 = t3304 * t261 * t6503;
    let t37880 = t571 * t1582 * t2096;
    (t37838, t37841, t37848, t37851, t37880)
}
