//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 455/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk455<F: Float>(t1809: F, t389: F, t1767: F, t388: F, t387: F, t1187: F, t1773: F, t358: F, t382: F, t1798: F, t1802: F, t1806: F) -> (F, F, F, F, F, F, F, F) {
    let t1810 = t1809 * t389;
    let t1812 = t388 * t1767;
    let t1813 = t387 * t1812;
    let t1814 = t1187 * t1813;
    let t1816 = t358 * t1773;
    let t1817 = t387 * t1816;
    let t1818 = t382 * t1817;
    let t1820 = t1798 / F::cast_from(16.0_f64) - t1802 / F::cast_from(16.0_f64) + t1806 / F::cast_from(24.0_f64) - t1810 / F::cast_from(256.0_f64) + t1814 / F::cast_from(256.0_f64) - t1818 / F::cast_from(192.0_f64);
    (t1810, t1812, t1813, t1814, t1816, t1817, t1818, t1820)
}
