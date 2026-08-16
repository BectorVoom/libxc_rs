//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2077/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2077(t5499: f64, t9929: f64, t172: f64, t5522: f64, t763: f64, t184: f64, t5398: f64, t607: f64, t4194: f64, t9864: f64, t9866: f64, t2752: f64, t5664: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16612 = 12.0_f64 * t9929 * t5499;
    let t16616 = t5522 * t172;
    let t16617 = t16616 * t763;
    let t16618 = 0.5848223622634646207e0_f64 * t16617;
    let t16619 = t184 * t5398;
    let t16620 = t16619 * t607;
    let t16622 = 12.0_f64 * t4194 * t16620;
    let t16623 = 0.11696447245269292414e1_f64 * t9864;
    let t16624 = 0.17315859105681463759e2_f64 * t9866;
    let t16625 = t5664 * t2752;
    (t16612, t16616, t16618, t16619, t16620, t16622, t16623, t16624, t16625)
}
