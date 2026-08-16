//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1209/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1209(t17541: f64, t5737: f64, t7285: f64, t1899: f64, t1901: f64, t683: f64, t7443: f64, t5738: f64, t7411: f64, t21004: f64, t21006: f64, t21008: f64, t21010: f64, t21012: f64, t21014: f64, t21016: f64, t21018: f64, t21021: f64, t21024: f64, t21027: f64, t21030: f64) -> (f64, f64, f64, f64) {
    let t21033 = 0.62071215503128080361e4_f64 * t17541 * t7285 * t5737;
    let t21037 = 0.48245938496077605201e2_f64 * t1899 * t7443 * t1901 * t683;
    let t21039 = 6.0_f64 * t7411 * t5738;
    let t21040 = t21004 + t21006 + t21008 + t21010 - t21012 - t21014 - t21016 - t21018 + t21021 + t21024 + t21027 + t21030 + t21033 - t21037 - t21039;
    (t21033, t21037, t21039, t21040)
}
