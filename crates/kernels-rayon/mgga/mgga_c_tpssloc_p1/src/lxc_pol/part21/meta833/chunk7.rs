//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2948/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2948(t13779: f64, t17171: f64, t2986: f64, t13784: f64, t17157: f64, t10190: f64, t17817: f64, t17769: f64, t2960: f64, t10224: f64, t5824: f64, t973: f64) -> (f64, f64, f64, f64, f64) {
    let t61391 = t2986 * t13779 * t17171;
    let t61394 = t2986 * t13784 * t17157;
    let t61397 = t2986 * t10190 * t17817;
    let t61405 = t2960 * t17769;
    let t61408 = t973 * t10224 * t5824;
    (t61391, t61394, t61397, t61405, t61408)
}
