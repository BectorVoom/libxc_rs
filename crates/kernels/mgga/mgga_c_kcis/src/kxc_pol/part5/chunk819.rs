//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 819/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk819<F: Float>(t3218: F, t6496: F, t1021: F, t1092: F, t1713: F, t4999: F, t1020: F, t1022: F, t6334: F, t6326: F, t2842: F, t2889: F, t6272: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6497 = t3218 * t6496;
    let t6498 = t1021 * t6497;
    let t6499 = t1092 * t6498;
    let t6501 = t4999 * t1713;
    let t6502 = t1020 * t6501;
    let t6504 = t1022 * t6334;
    let t6505 = t1021 * t6504;
    let t6506 = t1020 * t6505;
    let t6508 = t1022 * t6326;
    let t6509 = t1021 * t6508;
    let t6510 = t2842 * t6509;
    let t6517 = t2889 * t6272;
    (t6497, t6498, t6499, t6501, t6502, t6504, t6505, t6506, t6508, t6509, t6510, t6517)
}
