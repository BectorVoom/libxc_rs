//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 627/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk627(t1801: f64, t8672: f64, t5062: f64, t1869: f64, t2527: f64, t6697: f64, t1873: f64, t1224: f64, t4840: f64, t8510: f64, t1697: f64, t8514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8673 = t1801 * t8672;
    let t8674 = t5062 * t8673;
    let t8675 = t1869 * t8674;
    let t8677 = t6697 * t2527;
    let t8678 = t1873 * t8677;
    let t8679 = t1869 * t8678;
    let t8684 = t1224 * t4840 * t8510;
    let t8687 = t1224 * t1697 * t8514;
    (t8673, t8674, t8675, t8677, t8678, t8679, t8684, t8687)
}
