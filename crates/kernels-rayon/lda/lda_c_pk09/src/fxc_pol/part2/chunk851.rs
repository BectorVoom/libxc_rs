//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 851/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk851(t8092: f64, t984: f64, t7795: f64, t7797: f64, t7799: f64, t7801: f64, t7805: f64, t7809: f64, t7811: f64, t7814: f64, t7817: f64, t7834: f64, t7838: f64, t7842: f64, t7846: f64) -> (f64, f64) {
    let t8760 = t984 * t8092;
    let t8775 = -12.833936766110723_f64 * t7795 + 12.833936766110723_f64 * t7797 + 12.833936766110723_f64 * t7799 - 0.4266666666666667_f64 * t7801 - 0.64_f64 * t7805 - 0.64_f64 * t7809 - 0.64_f64 * t7811 - 0.64_f64 * t7814 - 0.64_f64 * t7817 - 0.64_f64 * t7834 - 9.625452574583042_f64 * t7838 + 9.625452574583042_f64 * t7842 + 9.625452574583042_f64 * t7846;
    (t8760, t8775)
}
