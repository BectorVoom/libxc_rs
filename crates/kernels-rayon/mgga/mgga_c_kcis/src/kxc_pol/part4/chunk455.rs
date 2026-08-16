//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 455/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk455(t1809: f64, t389: f64, t1767: f64, t388: f64, t387: f64, t1187: f64, t1773: f64, t358: f64, t382: f64, t1798: f64, t1802: f64, t1806: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1810 = t1809 * t389;
    let t1812 = t388 * t1767;
    let t1813 = t387 * t1812;
    let t1814 = t1187 * t1813;
    let t1816 = t358 * t1773;
    let t1817 = t387 * t1816;
    let t1818 = t382 * t1817;
    let t1820 = t1798 / 16.0_f64 - t1802 / 16.0_f64 + t1806 / 24.0_f64 - t1810 / 256.0_f64 + t1814 / 256.0_f64 - t1818 / 192.0_f64;
    (t1810, t1812, t1813, t1814, t1816, t1817, t1818, t1820)
}
