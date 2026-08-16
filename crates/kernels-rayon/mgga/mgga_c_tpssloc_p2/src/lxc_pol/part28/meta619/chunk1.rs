//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1939/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1939(t16311: f64, t3788: f64, t3791: f64, t6936: f64, t1339: f64, t1825: f64, t26288: f64, t3734: f64, t16314: f64, t26309: f64, t16227: f64, t22833: f64) -> (f64, f64, f64, f64) {
    let t91241 = t6936 * t3788 * t16311 * t3791;
    let t91256 = t26288 * t1339 * t1825 * t3734;
    let t91261 = t26309 * t16314;
    let t91263 = t22833 * t16227;
    (t91241, t91256, t91261, t91263)
}
