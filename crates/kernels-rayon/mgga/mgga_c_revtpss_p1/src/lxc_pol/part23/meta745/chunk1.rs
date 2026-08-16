//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2529/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2529(t10069: f64, t14537: f64, t10504: f64, t136: f64, t2457: f64, t4533: f64, t14473: f64, t9303: f64, t14477: f64, t2435: f64, t10073: f64, t14482: f64) -> (f64, f64, f64, f64, f64) {
    let t51703 = t10069 * t14537;
    let t51704 = 0.21951497276451705329e-1_f64 * t51703;
    let t51726 = t10504 * t4533 * t136 * t2457;
    let t51727 = 0.34697458558045176417e-2_f64 * t51726;
    let t51733 = t9303 * t14473;
    let t51741 = t2435 * t14477;
    let t51742 = 0.21951497276451705329e-1_f64 * t51741;
    let t51756 = t10073 * t14482;
    (t51704, t51727, t51733, t51742, t51756)
}
