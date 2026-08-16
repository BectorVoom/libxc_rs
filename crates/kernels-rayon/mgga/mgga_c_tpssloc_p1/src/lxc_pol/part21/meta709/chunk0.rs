//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2543/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2543(t10756: f64, t300: f64, t10828: f64, t2930: f64, t10390: f64, t14501: f64, t10422: f64, t13761: f64, t3070: f64, t1615: f64, t3120: f64, t3040: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49513 = t300 * t10756;
    let t49532 = t300 * t10828;
    let t49541 = t300 * t2930;
    let t49604 = t10390 * t14501;
    let t49607 = t3070 * t10422 * t13761;
    let t49616 = t1615 * t3120;
    let t49621 = t1615 * t3040;
    (t49513, t49532, t49541, t49604, t49607, t49616, t49621)
}
