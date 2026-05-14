//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 759/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk759<F: Float>(t8775: F, t8785: F, t8797: F, t8812: F, t804: F, t89: F, t2152: F, t609: F, t96: F, t839: F, t8092: F, t944: F, t8732: F, t891: F, t4587: F, t623: F, t896: F) -> (F, F, F, F, F) {
    let t8814 = t8775 + t8785 + t8797 + t8812;
    let t8815 = t8814 * t804;
    let t8816 = t8815 * t89;
    let t8819 = t2152 * t609;
    let t8820 = t96 * t8819;
    let t8821 = t839 * t8820;
    let t8829 = t944 * t8092;
    let t8836 = t891 * t8732 * t609;
    let t8837 = t4587 * t8836;
    let t8840 = t896 * t8732 * t623;
    (t8816, t8821, t8829, t8837, t8840)
}
