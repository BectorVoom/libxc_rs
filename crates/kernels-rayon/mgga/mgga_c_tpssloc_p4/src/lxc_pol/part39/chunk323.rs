//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 323/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk323(t1010: f64, t1011: f64, t361: f64, t363: f64) -> (f64, f64, f64, f64) {
    let t1012 = t1010 * t1011;
    let t1013 = t361 * t361;
    let t1014 = 1.0_f64 / t1013;
    let t1015 = t1014 * t363;
    (t1012, t1013, t1014, t1015)
}
