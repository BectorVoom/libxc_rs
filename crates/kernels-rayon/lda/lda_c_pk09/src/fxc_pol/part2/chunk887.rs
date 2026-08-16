//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 887/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk887(t7795: f64, t7797: f64, t7799: f64, t7801: f64, t7805: f64, t7809: f64, t7811: f64, t7814: f64, t7817: f64, t7834: f64, t7838: f64, t7842: f64, t7846: f64) -> f64 {
    let t9353 = -6.129211220482733_f64 * t7795 + 6.129211220482733_f64 * t7797 + 6.129211220482733_f64 * t7799 - 0.2037667917801196_f64 * t7801 - 0.3056501876701794_f64 * t7805 - 0.3056501876701794_f64 * t7809 - 0.3056501876701794_f64 * t7811 - 0.3056501876701794_f64 * t7814 - 0.3056501876701794_f64 * t7817 - 0.3056501876701794_f64 * t7834 - 4.59690841536205_f64 * t7838 + 4.59690841536205_f64 * t7842 + 4.59690841536205_f64 * t7846;
    t9353
}
