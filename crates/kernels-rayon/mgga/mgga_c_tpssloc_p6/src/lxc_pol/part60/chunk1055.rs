//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1055/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1055(t113875: f64, t116106: f64, t116115: f64, t117447: f64, t117451: f64, t117527: f64, t122945: f64, t122976: f64, t124755: f64, t124805: f64, t124807: f64, t124834: f64, t124838: f64, t126065: f64, t126073: f64, t129093: f64, t129096: f64, t1433: f64, t31864: f64, t32331: f64, t34126: f64, t5398: f64, t8308: f64, t8825: f64) -> f64 {
    let t130439 = -5.0_f64 / 72.0_f64 * t129093 * t8825 - 5.0_f64 / 36.0_f64 * t129096 * t8825 + 10.0_f64 / 27.0_f64 * t124805 + 10.0_f64 / 27.0_f64 * t124807 - 10.0_f64 / 3.0_f64 * t116106 * t117447 * t126065 + 10.0_f64 / 9.0_f64 * t31864 * t117451 * t126073 + 5.0_f64 / 9.0_f64 * t122976 * t34126 + 5.0_f64 / 3.0_f64 * t116115 * t113875 * t124755 * t1433 + 5.0_f64 / 9.0_f64 * t122945 * t34126 + 5.0_f64 / 18.0_f64 * t31864 * t8308 * t32331 * t5398 - 20.0_f64 / 9.0_f64 * t124834 + 20.0_f64 / 27.0_f64 * t124838 - t117527;
    t130439
}
