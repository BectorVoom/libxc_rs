//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1081/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1081(t12907: f64, t13475: f64, t13483: f64, t13491: f64, t2: f64, t873: f64, t584: f64, t265: f64, t16: f64, t4331: f64, t10723: f64, t4496: f64) -> (f64, f64, f64, f64, f64) {
    let t13493 = t12907 + t13475 + t13483 + t13491;
    let t13501 = t873 * t2;
    let t13503 = 2.0_f64 * t13501 * t584;
    let t13504 = t265 * t584;
    let t13506 = 3.0_f64 * t4331 * t16;
    let t13508 = t4496 * t10723;
    (t13493, t13503, t13504, t13506, t13508)
}
