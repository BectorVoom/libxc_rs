//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 440/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk440(t1001: f64, t1704: f64, t286: f64, t1700: f64, t285: f64, t989: f64, t991: f64, t1009: f64, t1022: f64, t1662: f64, t1021: f64, t1020: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1705 = t1001 * t1704;
    let t1706 = t286 * t1705;
    let t1709 = t989 + t991 * t1700 / 288.0_f64 - t285 * t1706 / 96.0_f64;
    let t1710 = t1709 * t1009;
    let t1713 = t1022 * t1662;
    let t1714 = t1021 * t1713;
    let t1715 = t1020 * t1714;
    (t1705, t1706, t1709, t1710, t1713, t1714, t1715)
}
