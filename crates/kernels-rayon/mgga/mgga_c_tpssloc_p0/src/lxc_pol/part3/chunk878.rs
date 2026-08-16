//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 878/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk878(t21: f64, t59: f64, t207: f64, t795: f64, t225: f64, t2711: f64, t2594: f64, t2690: f64, t841: f64, t812: f64, t849: f64, t2697: f64, t2707: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9580 = t59 * t21;
    let t9583 = 0.16435185185185185185e-1_f64 * t9580 * t207 * t795;
    let t9590 = t2711 * t225;
    let t9593 = t2594 * t225;
    let t9600 = t841 * t2690;
    let t9601 = t812 * t9600;
    let t9602 = t9601 * t849;
    let t9604 = t2697 * t2707;
    (t9580, t9583, t9590, t9593, t9601, t9602, t9604)
}
