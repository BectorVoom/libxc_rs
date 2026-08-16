//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1481/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1481(t28: f64, t265: f64, t504: f64, t76559: f64, t78240: f64, t78305: f64, t78342: f64, t79538: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t20217: f64, t20390: f64, t21076: f64, t22414: f64, t506: f64, t52: f64, t5398: f64, t5669: f64, t5966: f64, t6279: f64, t75912: f64, t77953: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t79541 = piecewise3(t505, t78240 + t78305 + t78342 + t79538, t76559);
    let t79553 = piecewise3(t401, t76559 * t28 / 2.0_f64 + 2.0_f64 * t21076 * t1649 + 3.0_f64 * t5669 * t5966 + 2.0_f64 * t1534 * t20390 + t265 * t77953 / 2.0_f64, t79541 * t52 / 2.0_f64 - 2.0_f64 * t22414 * t1409 - 3.0_f64 * t6279 * t5398 - 2.0_f64 * t1768 * t20217 - t506 * t75912 / 2.0_f64);
    t79553
}
