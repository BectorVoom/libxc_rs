//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 242/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk242(t666: f64, t670: f64, t612: f64, t616: f64, t626: f64, t636: f64, t653: f64, t676: f64, t681: f64, t687: f64, t110: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1029 = 4.59690841536205_f64 * t666;
    let t1030 = 3.0646056102413666_f64 * t670;
    let t1034 = 0.3056501876701794_f64 * t612;
    let t1035 = 0.2037667917801196_f64 * t616;
    let t1039 = t1029 + t1030 + 4.59690841536205_f64 * t676 + 4.59690841536205_f64 * t681 - 4.59690841536205_f64 * t687 + t1034 + t1035 + 0.3056501876701794_f64 * t626 + 0.3056501876701794_f64 * t636 - 0.3056501876701794_f64 * t653;
    let t1040 = t110 * t1039;
    let t1041 = t1040 * t89;
    (t1029, t1030, t1034, t1035, t1039, t1040, t1041)
}
