//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 951/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk951(t9296: f64, t9311: f64, t783: f64, t228: f64, t2766: f64, t2771: f64, t2772: f64, t2789: f64, t8524: f64, t899: f64, t9005: f64, t9007: f64, t9010: f64, t9017: f64, t9018: f64, t9021: f64, t906: f64, t9185: f64, t9267: f64, t9270: f64, t9272: f64, t9278: f64, t9281: f64) -> (f64, f64) {
    let t9312 = t9296 + t9311;
    let t9313 = t783 * t9312;
    let t9314 = t228 * t9005 - 3.0_f64 * t2766 * t2789 + 6.0_f64 * t2771 * t9021 + 6.0_f64 * t2772 * t9010 - t899 * t9185 - 3.0_f64 * t9007 * t906 - 6.0_f64 * t9017 * t9018 - t8524 - t9267 + t9270 + t9272 + t9278 - t9281 + t9313;
    (t9313, t9314)
}
