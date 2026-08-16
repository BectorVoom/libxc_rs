//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1315/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1315(t16060: f64, t5701: f64, t1889: f64, t4001: f64, t12185: f64, t16073: f64, t5709: f64, t1377: f64, t5713: f64, t1380: f64, t5477: f64, t16082: f64) -> (f64, f64, f64, f64, f64) {
    let t16874 = t5701 * t16060;
    let t16877 = t1889 * t4001;
    let t16878 = t12185 * t16877;
    let t16881 = t5709 * t16073;
    let t16884 = t5713 * t1377;
    let t16885 = t5477 * t1380;
    let t16886 = t16884 * t16885;
    let t16889 = t5709 * t16082;
    (t16874, t16878, t16881, t16886, t16889)
}
