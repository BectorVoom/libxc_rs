//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 679/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk679(t1398: f64, t5: f64, t753: f64, t1767: f64, t2021: f64, t1762: f64, t1978: f64, t1818: f64, t377: f64, t1983: f64, t1763: f64, t1949: f64) -> (f64, f64, f64, f64, f64) {
    let t5195 = t5 * t1398 * t753;
    let t5200 = t1767 * t2021;
    let t5202 = 0.97592231702715658578e-1_f64 * t1762 * t5200;
    let t5203 = t1767 * t1978;
    let t5205 = 0.48159733137676571079e0_f64 * t1762 * t5203;
    let t5206 = t377 * t1818;
    let t5207 = t5206 * t1983;
    let t5209 = 0.28518989949414381017e2_f64 * t1762 * t5207;
    let t5210 = t1763 * t1949;
    (t5195, t5202, t5205, t5209, t5210)
}
