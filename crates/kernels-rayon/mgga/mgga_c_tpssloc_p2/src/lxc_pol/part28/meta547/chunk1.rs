//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1816/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1816(t23168: f64, t23223: f64, t1882: f64, t81686: f64, t9537: f64, t213: f64, t225: f64, t852: f64, t23164: f64, t23204: f64, t23222: f64, t23238: f64) -> (f64, f64, f64, f64, f64) {
    let t82150 = t23168 * t23223;
    let t82153 = t81686 * t9537 * t1882;
    let t82159 = t213 * t852 * t225;
    let t82172 = t23164 * t23204 * t23222;
    let t82174 = t23168 * t23238;
    (t82150, t82153, t82159, t82172, t82174)
}
