//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1106/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1106<F: Float>(t1855: F, t1893: F, t2783: F, t1856: F, t5776: F, t7278: F, t1084: F, t5796: F, t17541: F, t5737: F, t7285: F, t1899: F, t1901: F, t683: F, t7443: F, t5738: F, t7411: F) -> (F, F, F, F, F, F) {
    let t21024 = 6.0 * t1855 * t2783 * t1893;
    let t21027 = 0.28947563097646563121e3 * t5776 * t7278 * t1856;
    let t21030 = 2.0 * t1855 * t1084 * t5796;
    let t21033 = 0.62071215503128080361e4 * t17541 * t7285 * t5737;
    let t21037 = 0.48245938496077605201e2 * t1899 * t7443 * t1901 * t683;
    let t21039 = 6.0 * t7411 * t5738;
    (t21024, t21027, t21030, t21033, t21037, t21039)
}
