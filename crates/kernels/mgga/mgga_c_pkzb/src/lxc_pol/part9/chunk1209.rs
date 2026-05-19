//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1209/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1209<F: Float>(t17541: F, t5737: F, t7285: F, t1899: F, t1901: F, t683: F, t7443: F, t5738: F, t7411: F, t21004: F, t21006: F, t21008: F, t21010: F, t21012: F, t21014: F, t21016: F, t21018: F, t21021: F, t21024: F, t21027: F, t21030: F) -> (F, F, F, F) {
    let t21033 = F::cast_from(0.62071215503128080361e4_f64) * t17541 * t7285 * t5737;
    let t21037 = F::cast_from(0.48245938496077605201e2_f64) * t1899 * t7443 * t1901 * t683;
    let t21039 = F::new(6.0) * t7411 * t5738;
    let t21040 = t21004 + t21006 + t21008 + t21010 - t21012 - t21014 - t21016 - t21018 + t21021 + t21024 + t21027 + t21030 + t21033 - t21037 - t21039;
    (t21033, t21037, t21039, t21040)
}
