//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 868/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk868(t119: f64, t5187: f64, t210: f64, t225: f64, t5210: f64, t554: f64, t1814: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t5226 = t119 * t5187;
    let t5227 = t210 * t5226;
    let t5230 = t5210 * t225;
    let t5231 = t5230 * t554;
    let t5234 = t1814 * t68;
    (t5226, t5227, t5230, t5231, t5234)
}
