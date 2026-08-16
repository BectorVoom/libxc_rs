//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 907/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk907(t16: f64, t2: f64, t591: f64, t9: f64, t21: f64, t587: f64, t14: f64, t598: f64, t2230: f64, t594: f64, t2229: f64, t3: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9212 = t2 * t16;
    let t9214 = t9 * t591;
    let t9216 = t587 * t21;
    let t9218 = t14 * t598;
    let t9220 = t594 * t2230;
    let t9222 = t2229 * t3;
    (t9212, t9214, t9216, t9218, t9220, t9222)
}
