//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1616/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1616(t248: f64, t3041: f64, t3101: f64, t3039: f64, t3108: f64, t3113: f64, t10889: f64, t3128: f64, t3033: f64) -> (f64, f64, f64, f64, f64) {
    let t10895 = t248 * t3101 * t3041;
    let t10896 = t3039 * t10895;
    let t10898 = t3113 * t3108;
    let t10903 = t3128 * t10889;
    let t10904 = t3033 * t10903;
    (t10895, t10896, t10898, t10903, t10904)
}
