//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 818/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk818(t1164: f64, t4879: f64, t1694: f64, t3400: f64, t1155: f64, t3403: f64, t1171: f64, t1706: f64, t1420: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4881 = 0.5848223622634646207e0_f64 * t1164 * t4879;
    let t4882 = t3400 * t1694;
    let t4883 = t3403 * t1155;
    let t4884 = t4882 * t4883;
    let t4886 = 0.17315859105681463759e2_f64 * t1164 * t4884;
    let t4887 = t1706 * t1171;
    let t4889 = t1420 * t972;
    (t4881, t4882, t4883, t4884, t4886, t4887, t4889)
}
