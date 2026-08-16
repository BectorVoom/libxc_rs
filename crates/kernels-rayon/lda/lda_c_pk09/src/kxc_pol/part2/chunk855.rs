//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 855/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk855(t8775: f64, t8785: f64, t8797: f64, t8812: f64, t804: f64, t89: f64, t2152: f64, t609: f64, t96: f64, t839: f64, t8092: f64, t944: f64) -> (f64, f64, f64) {
    let t8814 = t8775 + t8785 + t8797 + t8812;
    let t8815 = t8814 * t804;
    let t8816 = t8815 * t89;
    let t8819 = t2152 * t609;
    let t8820 = t96 * t8819;
    let t8821 = t839 * t8820;
    let t8829 = t944 * t8092;
    (t8816, t8821, t8829)
}
