//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1302/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1302(t3: f64, t30094: f64, t112: f64, t8199: f64, t111: f64, t2205: f64, t671: f64, t8189: f64, t2199: f64, t2363: f64, t12521: f64, t12524: f64, t1401: f64, t16535: f64, t2319: f64, t30071: f64, t3938: f64, t3941: f64, t577: f64, t8207: f64, t8212: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30095 = t3 * t30094;
    let t30109 = t8199 * t112;
    let t30112 = t2205 * t111;
    let t30125 = t8189 * t671;
    let t30128 = t2199 * t2363;
    let t30133 = 0.45e1_f64 * t30094 * t577 + 27.0_f64 * t30109 * t671 + 27.0_f64 * t30112 * t2319 + 0.135e2_f64 * t8207 * t2363 + 0.135e2_f64 * t12521 * t2199 + 54.0_f64 * t12524 * t8212 + 27.0_f64 * t3938 * t8189 + 27.0_f64 * t16535 * t2199 + 54.0_f64 * t3941 * t30125 + 27.0_f64 * t3941 * t30128 + 0.135e2_f64 * t1401 * t30071;
    (t30095, t30109, t30112, t30125, t30128, t30133)
}
