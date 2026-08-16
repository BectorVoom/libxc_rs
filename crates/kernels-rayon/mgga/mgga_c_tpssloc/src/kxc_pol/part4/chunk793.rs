//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 793/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk793(t2932: f64, t5811: f64, t959: f64, t2980: f64, t5392: f64, t2979: f64, t4514: f64, t4531: f64, t2994: f64, t977: f64, t5398: f64, t978: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5812 = t5811 * t2932;
    let t5814 = 0.17315859105681463759e2_f64 * t959 * t5812;
    let t5817 = t2980 * t5392;
    let t5818 = t2979 * t5817;
    let t5821 = t4531 * t4514;
    let t5824 = t2994 * t5392;
    let t5825 = t977 * t5824;
    let t5828 = t978 * t5398;
    (t5812, t5814, t5817, t5818, t5821, t5824, t5825, t5828)
}
