//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1036/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1036(t4701: f64, t799: f64, t750: f64, t14029: f64, t778: f64, t1373: f64, t1375: f64, t14268: f64, t14274: f64, t14282: f64, t14285: f64, t222: f64, t224: f64, t3650: f64, t3656: f64, t3658: f64, t3661: f64, t4748: f64, t4752: f64, t4755: f64, t776: f64, t779: f64) -> f64 {
    let t14290 = t799 * t4701;
    let t14291 = t14290 * t750;
    let t14294 = t778 * t14029;
    let t14297 = 6.0_f64 * t1373 * t3661 + 6.0_f64 * t1375 * t3650 - t14268 * t224 - 24.0_f64 * t14274 * t3658 + 60.0_f64 * t14282 * t3656 - 24.0_f64 * t14285 * t3656 - 12.0_f64 * t14291 * t3656 + 3.0_f64 * t14294 * t222 + 3.0_f64 * t4748 * t779 - 12.0_f64 * t4752 * t776 + 3.0_f64 * t4755 * t776;
    t14297
}
