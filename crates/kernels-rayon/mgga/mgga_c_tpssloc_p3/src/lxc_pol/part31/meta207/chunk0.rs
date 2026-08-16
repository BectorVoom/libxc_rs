//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 918/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk918(t252: f64, t5558: f64, t1492: f64, t1519: f64, t119: f64, t5527: f64, t210: f64, t5544: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5559 = t5558 * t252;
    let t5561 = t1492 * t1519;
    let t5567 = t119 * t5527;
    let t5568 = t210 * t5567;
    let t5571 = t119 * t5544;
    let t5572 = t210 * t5571;
    let t5575 = t5558 * t225;
    (t5559, t5561, t5567, t5568, t5571, t5572, t5575)
}
