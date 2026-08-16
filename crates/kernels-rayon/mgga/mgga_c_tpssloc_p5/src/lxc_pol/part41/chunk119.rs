//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 119/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk119(t273: f64, t276: f64, t279: f64, t285: f64) -> (f64, f64, f64) {
    let t320 = 0.51785e1_f64 * t276 + 0.905775e0_f64 * t273 + 0.1100325e0_f64 * t279 + 0.1241775e0_f64 * t285;
    let t323 = 1.0_f64 + 0.29608749977793437516e2_f64 / t320;
    let t324 = f64::ln(t323);
    (t320, t323, t324)
}
