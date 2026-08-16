//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1020/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1020(t113836: f64, t113875: f64, t116088: f64, t116096: f64, t116099: f64, t116111: f64, t116115: f64, t116119: f64, t117499: f64, t117503: f64, t117516: f64, t117518: f64, t117527: f64, t2250: f64, t2303: f64, t31864: f64, t32331: f64, t32333: f64, t32338: f64, t63: f64, t641: f64, t8308: f64, t8513: f64, t8663: f64, t8825: f64) -> f64 {
    let t117528 = -5.0_f64 / 36.0_f64 * t8663 * t8513 * t32338 * t2303 - 5.0_f64 / 72.0_f64 * t116096 * t8825 - 5.0_f64 / 36.0_f64 * t116099 * t8825 - 40.0_f64 / 27.0_f64 * t117499 + 5.0_f64 / 9.0_f64 * t116111 * t32333 + 5.0_f64 / 3.0_f64 * t116115 * t113875 * t117503 * t641 + 5.0_f64 / 9.0_f64 * t116119 * t32333 + 5.0_f64 / 18.0_f64 * t31864 * t8308 * t32331 * t2250 - 5.0_f64 / 72.0_f64 * t116088 * t8825 + 10.0_f64 / 27.0_f64 * t117516 + 10.0_f64 / 27.0_f64 * t117518 - 5.0_f64 / 36.0_f64 * t8663 * t8513 * t113836 * t63 - t117527;
    t117528
}
