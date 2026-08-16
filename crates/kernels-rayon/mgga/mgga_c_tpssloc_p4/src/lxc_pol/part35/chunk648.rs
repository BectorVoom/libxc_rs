//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 648/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk648(t3403: f64, t6105: f64, t1164: f64, t338: f64, t5416: f64, t3441: f64, t5392: f64, t3440: f64, t4904: f64, t4919: f64, t3455: f64, t1177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6106 = t6105 * t3403;
    let t6108 = 0.17315859105681463759e2_f64 * t1164 * t6106;
    let t6109 = t5416 * t338;
    let t6119 = t3441 * t5392;
    let t6120 = t3440 * t6119;
    let t6123 = t4919 * t4904;
    let t6126 = t3455 * t5392;
    let t6127 = t1177 * t6126;
    (t6106, t6108, t6109, t6119, t6120, t6123, t6126, t6127)
}
