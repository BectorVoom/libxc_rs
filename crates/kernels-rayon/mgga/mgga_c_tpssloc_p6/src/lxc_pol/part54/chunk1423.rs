//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1423/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1423(t33240: f64, t6883: f64, t225: f64, t33267: f64, t115567: f64, t120542: f64, t120547: f64, t120551: f64, t120552: f64, t120553: f64, t120556: f64, t1385: f64, t1386: f64, t26224: f64, t26366: f64, t27068: f64, t31601: f64, t5321: f64, t6993: f64, t7199: f64, t7728: f64, t93319: f64) -> f64 {
    let t122295 = t6883 * t33240;
    let t122297 = t33267 * t225;
    let t122299 = -t27068 * t6993 + t120542 + 2.0_f64 * t26366 * t7199 + 24.0_f64 * t26224 * t93319 * t7728 * t1385 - t120547 + 2.0_f64 * t5321 * t31601 - t120551 + t115567 + 0.19190897446562641759e-1_f64 * t122295 - t120552 - t122297 * t1386 + t120553 + t120556;
    t122299
}
