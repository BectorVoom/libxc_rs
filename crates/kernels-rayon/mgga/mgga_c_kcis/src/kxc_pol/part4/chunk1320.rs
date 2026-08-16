//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1320/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1320(t12194: f64, t16950: f64, t1930: f64, t3967: f64, t4007: f64, t5726: f64, t613: f64, t1377: f64, t3977: f64, t3754: f64, t1380: f64, t5654: f64) -> (f64, f64, f64, f64, f64) {
    let t16951 = t12194 * t16950;
    let t16954 = t1930 * t3967;
    let t16958 = t5726 * t4007;
    let t16959 = t613 * t16958;
    let t16962 = t3977 * t1377;
    let t16963 = t16962 * t3754;
    let t16964 = t5654 * t1380;
    (t16951, t16954, t16959, t16963, t16964)
}
