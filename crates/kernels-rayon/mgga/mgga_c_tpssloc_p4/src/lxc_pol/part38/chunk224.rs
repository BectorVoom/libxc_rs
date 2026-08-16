//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 224/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk224(t510: f64, t671: f64, t3: f64, t60: f64) -> (f64, f64) {
    let t672 = t510 * t671;
    let t675 = t60 * t3;
    let t676 = 1.0_f64 / t675;
    (t672, t676)
}
