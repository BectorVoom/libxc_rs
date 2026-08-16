//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2332/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2332(t100822: f64, t100864: f64, t96749: f64, t96793: f64, t96840: f64, t97814: f64, t97859: f64, t97906: f64, t16524: f64, t26545: f64, t1873: f64, t66958: f64) -> (f64, f64, f64) {
    let t100867 = t96749 + t96793 + t96840 + t97814 + t97859 + t97906 + t100822 + t100864;
    let t100871 = 54.0_f64 * t16524 * t26545;
    let t100873 = 0.135e2_f64 * t66958 * t1873;
    (t100867, t100871, t100873)
}
