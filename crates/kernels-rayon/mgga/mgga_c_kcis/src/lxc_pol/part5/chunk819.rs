//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 819/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk819(t3218: f64, t6496: f64, t1021: f64, t1092: f64, t1713: f64, t4999: f64, t1020: f64, t1022: f64, t6334: f64, t6326: f64, t2842: f64, t2889: f64, t6272: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
