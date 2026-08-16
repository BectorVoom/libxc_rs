//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2178/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2178(t1307: f64, t22635: f64, t567: f64, t6330: f64, t90591: f64, t28199: f64, t6897: f64, t794: f64, t1985: f64, t20009: f64, t214: f64, t225: f64) -> (f64, f64, f64) {
    let t97588 = t90591 * t22635 * t567 * t6330 * t1307;
    let t97599 = t6897 * t794 * t28199;
    let t97604 = t1985 * t214 * t20009 * t225 * t567;
    (t97588, t97599, t97604)
}
