//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1315/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1315(t18280: f64, t3531: f64, t6556: f64, t6552: f64, t3362: f64, t5825: f64, t606: f64, t3417: f64, t141: f64, t1121: f64, t18281: f64, t1145: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20256 = -t18280;
    let t20261 = 0.17315859105681463759e2_f64 * t3531 * t6556;
    let t20263 = 0.5848223622634646207e0_f64 * t3531 * t6552;
    let t20265 = t3362 * t5825;
    let t20266 = t20265 * t606;
    let t20267 = t3417 * t20266;
    let t20268 = t141 * t20267;
    let t20272 = t1121 * t18281;
    let t20273 = t1145 * t20272;
    (t20256, t20261, t20263, t20266, t20268, t20272, t20273)
}
