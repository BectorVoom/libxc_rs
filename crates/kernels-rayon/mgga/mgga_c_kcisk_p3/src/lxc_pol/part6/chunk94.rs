//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 94/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk94(t303: f64, t306: f64, t309: f64, t315: f64, t325: f64, t323: f64, t45: f64) -> (f64, f64, f64, f64, f64) {
    let t330 = 0.51785e1_f64 * t306 + 0.905775e0_f64 * t303 + 0.1100325e0_f64 * t309 + 0.1241775e0_f64 * t315;
    let t333 = 1.0_f64 + 0.29608574643216675549e2_f64 / t330;
    let t334 = f64::ln(t333);
    let t335 = t325 * t334;
    let t338 = -t323 + 0.19751789702565206229e-1_f64 * t45 * t335;
    (t330, t333, t334, t335, t338)
}
