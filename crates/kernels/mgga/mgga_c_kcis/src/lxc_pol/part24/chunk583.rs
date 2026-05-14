//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 583/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk583<F: Float>(t3218: F, t6496: F, t1021: F, t1092: F, t1713: F, t4999: F, t1020: F, t1022: F, t6334: F, t6326: F, t2842: F, t2889: F, t6272: F, t2888: F, t1662: F, t1704: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t6518 = t2888 * t6517;
    let t6521 = t1662 * t1704;
    (t6497, t6498, t6499, t6501, t6502, t6504, t6505, t6506, t6508, t6509, t6510, t6517, t6518, t6521)
}
