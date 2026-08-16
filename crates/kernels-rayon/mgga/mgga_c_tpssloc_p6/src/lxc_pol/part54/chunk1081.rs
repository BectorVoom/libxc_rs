//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1081/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1081(t27095: f64, t27113: f64, t1378: f64, t1375: f64, t1386: f64, t16022: f64, t16439: f64, t1843: f64, t2092: f64, t22676: f64, t24095: f64, t26475: f64, t27067: f64, t27068: f64, t27070: f64, t3758: f64, t3882: f64, t5215: f64, t5321: f64, t568: f64, t7199: f64, t7214: f64, t7937: f64) -> (f64, f64, f64) {
    let t27114 = t27095 + t27113;
    let t27115 = t1378 * t27114;
    let t27127 = -t27067 - t27068 * t1386 + t27070 * t568 - t1375 * t27115 - t3882 * t7937 - t16022 * t2092 - 0.82246703342411321825e-2_f64 * t26475 - t16439 * t2092 + 2.0_f64 * t5215 * t7199 - t5321 * t7214 - t3758 * t7937 + 0.82246703342411321825e-2_f64 * t22676 - t24095 * t1843;
    (t27114, t27115, t27127)
}
