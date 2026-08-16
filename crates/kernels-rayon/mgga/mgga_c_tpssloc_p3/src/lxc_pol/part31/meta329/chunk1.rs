//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1227/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1227(t1471: f64, t32: f64, t4095: f64, t67: f64, t758: f64, t118: f64, t1474: f64, t2375: f64, t4094: f64, t706: f64, t4162: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t13115 = t32 * t1471;
    let t13119 = t4095 * t67;
    let t13121 = 0.36622894612013090108e-3_f64 * t13119 * t758;
    let t13123 = t1474 * t118;
    let t13124 = t13123 * t2375;
    let t13133 = t706 * t4094;
    let t13176 = t4162 * t68;
    (t13115, t13121, t13124, t13133, t13176)
}
