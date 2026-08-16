//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 900/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk900(t2199: f64, t671: f64, t1401: f64, t3938: f64, t3941: f64, t577: f64, t8189: f64, t8199: f64, t8207: f64, t1774: f64, t1453: f64, t8180: f64) -> (f64, f64, f64, f64) {
    let t8212 = t2199 * t671;
    let t8217 = 0.45e1_f64 * t8199 * t577 + 0.135e2_f64 * t8207 * t671 + 0.135e2_f64 * t3938 * t2199 + 27.0_f64 * t3941 * t8212 + 0.135e2_f64 * t1401 * t8189;
    let t8260 = t1774 * t2199;
    let t8262 = t8180 * t1453;
    (t8212, t8217, t8260, t8262)
}
