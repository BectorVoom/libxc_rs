//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 779/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk779(t577: f64, t671: f64, t7014: f64, t7017: f64, t7019: f64, t7415: f64, t7423: f64, t33: f64, t3953: f64) -> (f64, f64) {
    let t7426 = 0.45e1_f64 * t7415 * t577 + 0.135e2_f64 * t7423 * t671 + t7014 + t7017 + t7019;
    let t7428 = t3953 * t33;
    (t7426, t7428)
}
