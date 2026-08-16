//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1235/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1235(t1834: f64, t5210: f64, t1807: f64, t5318: f64, t1842: f64, t5353: f64, t3887: f64, t1814: f64, t5333: f64, t1338: f64, t6434: f64, t1352: f64) -> (f64, f64, f64, f64, f64) {
    let t19635 = t5210 * t1834;
    let t19644 = t1807 * t5318;
    let t19647 = t1842 * t5353;
    let t19648 = t3887 * t19647;
    let t19654 = t1814 * t5333;
    let t19657 = t1338 * t6434;
    let t19658 = t19657 * t1352;
    (t19635, t19644, t19648, t19654, t19658)
}
