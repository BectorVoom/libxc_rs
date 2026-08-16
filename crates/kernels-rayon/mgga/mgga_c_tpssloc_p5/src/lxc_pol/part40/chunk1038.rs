//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1038/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1038(t5575: f64, t68: f64, t4234: f64, t4295: f64, t12850: f64, t12860: f64, t16577: f64, t16578: f64, t16581: f64, t16582: f64, t16588: f64, t16612: f64, t9457: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64, t9724: f64) -> (f64, f64, f64) {
    let t16673 = t5575 * t68;
    let t16679 = t4295 * t4234;
    let t16684 = t12850 + t16577 + t16578 - t9457 + t16581 - t12860 + t16582 - t9469 + t16588 + t9476 + t9484 - t9496 - t9715 + t16612 + t9724;
    (t16673, t16679, t16684)
}
